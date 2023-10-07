mod cli;
mod page_walker;
mod range;
mod utils;
mod win32kernel_ext;

use anyhow::{Context, Result};
use clap::Parser;

use memflow::prelude::*;
use memflow_win32::prelude::*;

use aobscan::PatternBuilder;

use crate::cli::Cli;
use crate::page_walker::PageWalker;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(&cli) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: &Cli) -> Result<()> {
    let pattern = PatternBuilder::from_hex_string(&cli.pattern)?.build();
    let patch = cli
        .patch
        .as_ref()
        .map(|patch| hex::decode(patch).context("the patch provided is not in a valid format"))
        .transpose()?;

    // QemuProcFs has advantage of being highly portable, and users won't even need to install any kernel module.
    let connector_args = ConnectorArgs::new(cli.target.as_deref(), Args::default(), None);
    let connector = memflow_qemu::create_connector(&connector_args)
        .context("qemu connector creation failed, is the target running?")?;

    let mut kernel = Win32Kernel::builder(connector)
        .build_default_caches()
        .build()
        .map_err(|e| anyhow::anyhow!("failed to build win32kernel: {}", e))?;
    anyhow::ensure!(
        kernel.info().arch == ArchitectureIdent::X86(64, false),
        "target architecture is not x86_64"
    );

    let winver = kernel.kernel_info.kernel_winver;
    println!(
        "connected to target, winver: {}.{}.{}",
        winver.major_version(),
        winver.minor_version(),
        winver.build_number()
    );

    println!("generating pageset, this may take a while");
    let pages = PageWalker::new(kernel.clone())?.scan()?;
    anyhow::ensure!(
        pages.len() > 0,
        "target memory page iteration returned nothing"
    );

    let total_size = pages.iter().map(|r| r.size()).sum();
    let total_size_str = utils::format_bytes(total_size);
    println!(
        "found {} pages for a total span of {}",
        pages.len(),
        total_size_str
    );

    println!("========================");
    for page in pages {
        let buffer = kernel.phys_view().read_raw(page.address(), page.size())?;
        let mut offsets = Vec::new();
        pattern.scan(buffer.as_slice(), |offset| {
            offsets.push(offset);
            true
        });

        for offset in offsets {
            let address = page.address() + offset;
            if let Some(patch) = &patch {
                if kernel.phys_view().write_raw(address, patch).is_err() {
                    eprintln!("0x{} -> err", address);
                } else {
                    println!("0x{} -> ok", address);
                }
            } else {
                println!("0x{}", address);
            }
        }
    }

    Ok(())
}
