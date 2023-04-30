use std::error::Error;
use std::result::Result;

use aobscan::{Pattern, PatternBuilder};
use clap::Parser;
use colored::Colorize;
use memflow::prelude::*;
use memflow_win32::prelude::*;

use args::Args;

mod args;
mod memory;
mod utils;

fn main() {
    let args = Args::parse();
    if let Err(e) = run(&args) {
        if !args.raw_output {
            eprintln!("💥 {}", e.to_string().red().bold());
        } else {
            eprintln!("{}", e);
        }

        std::process::exit(1);
    }
}

fn run(args: &Args) -> Result<(), Box<dyn Error>> {
    let pattern = create_pattern(&args.pattern, args.threads)?;
    let patch = create_patch(&args.patch)?;

    let connector_args = ConnectorArgs::new(
        args.target.as_deref(),
        memflow::prelude::Args::default(),
        None,
    );

    let connector = memflow_qemu::create_connector(&connector_args)
        .map_err(|_| "Could not create connector, is the target running?")?;

    let mut kernel = Win32Kernel::builder(connector)
        .build_default_caches()
        .build()
        .map_err(|e| format!("Unable to create Win32 kernel: {}", e))?;

    let ntoskrnl = kernel
        .kernel_process_info()
        .map_err(|e| format!("Unable to get ntoskrnl.sys: {}", e))?;

    // Where the magic happens.
    println!("☕ Time for a coffee, this is going to take a while");
    memory::begin_phys_scan(
        &mut kernel,
        ntoskrnl,
        pattern,
        patch.as_deref(),
        args.raw_output,
    )
}

fn create_pattern(pattern: &str, threads: Option<usize>) -> Result<Pattern, Box<dyn Error>> {
    // Detect if the giiven pattern is in IDA-style or HEX-raw format.
    // This is done by checking if the pattern contains a space.
    let builder = if pattern.contains(' ') {
        PatternBuilder::from_ida_style(&pattern)
    } else {
        PatternBuilder::from_hex_string(&pattern)
    }
    .map_err(|e| format!("Failed to parse pattern: {}", e))?;

    // Build the pattern from the builder with the given number of threads.
    Ok(if let Some(threads) = threads {
        if threads != 0 {
            // If the specified number of threads is different from 0,
            // use that to scan the memory.
            builder
                .with_threads(threads)
                .map_err(|e| format!("Failed to set number of threads: {}", e))?
        } else {
            // If the specified number of threads is 0, use all the available cores.
            // (maximum parallelism)
            builder.with_all_threads()
        }
    } else {
        // If the number of threads is not specified, use aall the available cores.
        // (maximum parallelism)
        builder.with_all_threads()
    }
    .build())
}

fn create_patch(patch: &Option<String>) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    match patch {
        Some(patch_str) => {
            // This should be enough to support patches in the following format:
            //   - 488BC0252162
            //   - 48 8B C0 25 21 62
            //   - \x48\x8B\xC0\x25\x21\x62
            let patch_copy = patch_str.clone();
            let patch_hex = patch_copy.replace(" ", "").replace(r"\x", "");
            let patch_bytes = hex::decode(patch_hex)
                .map_err(|_| "The patch provided is not in a supported format")?;

            // Empty patches are handled the same way as None patches.
            Ok(if patch_bytes.is_empty() {
                None
            } else {
                Some(patch_bytes)
            })
        }
        None => Ok(None),
    }
}
