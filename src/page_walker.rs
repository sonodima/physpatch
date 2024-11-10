use std::collections::HashSet;

use anyhow::{Context, Result};

use ::x86::current::paging::*;
use memflow::prelude::*;
use memflow_win32::prelude::*;

use crate::range::Range;
use crate::win32kernel_ext::Win32KernelExt;

// Reference:
// https://www.iaik.tugraz.at/teaching/materials/os/tutorials/paging-on-intel-x86-64
pub struct PageWalker<P, V> {
    kernel: Win32Kernel<P, V>,
    ntoskrnl: Win32ProcessInfo,
}

impl<P, V> PageWalker<P, V>
where
    P: 'static + PhysicalMemory + Clone,
    V: 'static + VirtualTranslate2 + Clone,
{
    pub fn new(mut kernel: Win32Kernel<P, V>) -> Result<Self> {
        let walker = Self {
            ntoskrnl: kernel
                .kernel_process_info()
                .context("failed to obtain ntoskrnl.exe info")?,
            kernel,
        };

        Ok(walker)
    }

    pub fn scan(&mut self) -> Result<Vec<Range>> {
        let pml4: PML4 = unsafe { self.kernel.phys_read_unchecked(self.ntoskrnl.base_info.dtb1) }
            .context("failed to read the system's page map")?;

        let mut pageset = HashSet::new();
        unsafe { self.walk_pml4(&pml4, &mut pageset) };

        let mut pages: Vec<Range> = pageset.into_iter().collect();
        pages.sort_unstable();
        Ok(pages)
    }

    unsafe fn walk_pml4(&mut self, pml4: &PML4, pageset: &mut HashSet<Range>) {
        for pml4e in pml4 {
            if pml4e.is_present() && !pml4e.address().is_zero() {
                let address: Address = pml4e.address().as_u64().into();
                if let Ok(pdpt) = self.kernel.phys_read_unchecked(address) {
                    self.walk_pdpt(&pdpt, pageset);
                }
            }
        }
    }

    unsafe fn walk_pdpt(&mut self, pdpt: &PDPT, pageset: &mut HashSet<Range>) {
        for pdpte in pdpt {
            if pdpte.is_present() && !pdpte.address().is_zero() {
                let address: Address = pdpte.address().as_u64().into();

                if pdpte.flags().contains(PDPTFlags::PS) {
                    // Huge Page (1GB)
                    let range = Range::new(address, 0x40000000);
                    pageset.insert(range);
                } else if let Ok(pd) = self.kernel.phys_read_unchecked(address) {
                    self.walk_pd(&pd, pageset);
                }
            }
        }
    }

    unsafe fn walk_pd(&mut self, pd: &PD, pageset: &mut HashSet<Range>) {
        for pde in pd {
            if pde.is_present() && !pde.address().is_zero() {
                let address: Address = pde.address().as_u64().into();

                if pde.flags().contains(PDFlags::PS) {
                    // Large Page (2MB)
                    let range = Range::new(address, 0x200000);
                    pageset.insert(range);
                } else if let Ok(pt) = self.kernel.phys_read_unchecked(address) {
                    self.walk_pt(&pt, pageset);
                }
            }
        }
    }

    fn walk_pt(&mut self, pt: &PT, pageset: &mut HashSet<Range>) {
        for pte in pt {
            if pte.is_present() && !pte.address().is_zero() {
                let address: Address = pte.address().as_u64().into();
                // Regular Page (4KB)
                let range = Range::new(address, 0x1000);
                pageset.insert(range);
            }
        }
    }
}
