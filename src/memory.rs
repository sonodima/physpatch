use std::error::Error;
use std::ops::Add;
use std::result::Result;

use aobscan::Pattern;
use x86::current::paging::{PDFlags, PDPTFlags, PD, PDPT, PML4, PT};

use memflow::prelude::*;
use memflow_win32::prelude::*;

use crate::utils::MemoryExtension;

pub fn begin_phys_scan<P: 'static + PhysicalMemory, V: 'static + VirtualTranslate2>(
    kernel: &mut Win32Kernel<P, V>,
    ntoskrnl: Win32ProcessInfo,
    pattern: Pattern,
    patch: Option<&[u8]>,
    raw_output: bool,
) -> Result<(), Box<dyn Error>> {
    let pml4: PML4 = kernel
        .phys_read_unchecked(ntoskrnl.dtb)
        .map_err(|e| format!("Failed to read PML4: {}", e))?;

    walk_pml4es(kernel, &pml4, pattern, patch, raw_output);
    Ok(())
}

fn walk_pml4es<P: 'static + PhysicalMemory, V: 'static + VirtualTranslate2>(
    kernel: &mut Win32Kernel<P, V>,
    pml4: &PML4,
    pattern: Pattern,
    patch: Option<&[u8]>,
    raw_output: bool,
) {
    for pml4e in pml4 {
        if pml4e.address().is_zero() || !pml4e.is_present() {
            continue;
        }

        if let Ok(pdpt) = kernel.phys_read_unchecked(pml4e.address().as_u64().into()) {
            walk_pdptes(kernel, &pdpt, &pattern, patch, raw_output);
        }
    }
}

fn walk_pdptes<P: 'static + PhysicalMemory, V: 'static + VirtualTranslate2>(
    kernel: &mut Win32Kernel<P, V>,
    pdpt: &PDPT,
    pattern: &Pattern,
    patch: Option<&[u8]>,
    raw_output: bool,
) {
    for pdpte in pdpt {
        if pdpte.address().is_zero() || !pdpte.is_present() {
            continue;
        }

        if pdpte.flags().contains(PDPTFlags::PS) {
            scan(
                kernel,
                pdpte.address().as_u64().into(),
                1024 * 1024 * 1024,
                pattern,
                patch,
                raw_output,
            );

            continue;
        }

        if let Ok(pd) = kernel.phys_read_unchecked(pdpte.address().as_u64().into()) {
            walk_pdes(kernel, &pd, pattern, patch, raw_output);
        }
    }
}

fn walk_pdes<P: 'static + PhysicalMemory, V: 'static + VirtualTranslate2>(
    kernel: &mut Win32Kernel<P, V>,
    pd: &PD,
    pattern: &Pattern,
    patch: Option<&[u8]>,
    raw_output: bool,
) {
    for pde in pd {
        if pde.address().is_zero() || !pde.is_present() {
            continue;
        }

        if pde.flags().contains(PDFlags::PS) {
            scan(
                kernel,
                pde.address().as_u64().into(),
                1024 * 1024 * 2,
                pattern,
                patch,
                raw_output,
            );

            continue;
        }

        if let Ok(pt) = kernel.phys_read_unchecked(pde.address().as_u64().into()) {
            walk_ptes(kernel, &pt, pattern, patch, raw_output);
        }
    }
}

fn walk_ptes<P: 'static + PhysicalMemory, V: 'static + VirtualTranslate2>(
    kernel: &mut Win32Kernel<P, V>,
    pt: &PT,
    pattern: &Pattern,
    patch: Option<&[u8]>,
    raw_output: bool,
) {
    for pte in pt {
        if pte.address().is_zero() || !pte.is_present() {
            continue;
        }

        scan(
            kernel,
            pte.address().as_u64().into(),
            0x1000,
            pattern,
            patch,
            raw_output,
        );
    }
}

fn scan<P: 'static + PhysicalMemory, V: 'static + VirtualTranslate2>(
    kernel: &mut Win32Kernel<P, V>,
    pa: Address,
    size: usize,
    pattern: &Pattern,
    patch: Option<&[u8]>,
    raw_output: bool,
) {
    // If the read fails, just ignore it. It should be good enough.
    if let Ok(data) = kernel.phys_view().read_raw(pa, size) {
        // Generate a list of matches for the pattern. We don't do the patching
        // in here because we can't safely share the kernel between threads.
        // (unless we wanted to clone the connector, but since the main bottleneck
        // is the pattern scan, we can avoid it)
        let mut matches = vec![];
        pattern.scan(&data, |offset| {
            matches.push(offset);
            true // Return true to continue scanning for other matches
        });

        // Patch all the results found by the pattern scan.
        // TODO: we could perform the write operations with phys_write_raw_iter?
        // Would it be faster? I have no clue.
        matches
            .iter()
            .map(|offset| pa.add(*offset))
            .for_each(|address| {
                if let Some(patch_data) = patch {
                    // If we want to apply a patch, do that in here.
                    let patched = kernel.phys_view().write_raw(address, patch_data).is_ok();
                    if !raw_output {
                        println!(
                            " » 0x{:#02X} › {}",
                            address,
                            if patched { "✅" } else { "🟥" }
                        );
                    } else {
                        println!(
                            "0x{:#02X} {}",
                            address,
                            if patched { "OK" } else { "FAILED" }
                        );
                    }
                } else {
                    // This is the case in which we just want to find the data in memory
                    // without performing any patch.
                    if !raw_output {
                        println!(" » 0x{:#02X}", address,);
                    } else {
                        println!("0x{:#02X}", address);
                    }
                }
            });
    }
}
