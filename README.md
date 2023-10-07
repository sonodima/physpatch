<h1 align="center">PhysPatch 🩹</h1>

<div align="center">
  <a href="https://github.com/sonodima/physpatch/actions?workflow=CI">
    <img src="https://github.com/sonodima/physpatch/workflows/CI/badge.svg"/>
  </a>
  <img src="https://img.shields.io/badge/license-MIT-blue.svg"/>
</div>

<br>

PhysPatch performs **physical memory** scans and patches of the entire **Windows kernel.**

## Usage

Scan for "488b??????????48" in the virtual machine named "TargetVM" and write
"488b0000" to all matches:

```sh
physpatch -t "TargetVM" -p "488b0000" -- "488b??????????48"
```

Scan for "488b?????48" in the first virtual machine found, without performing
any patching:

```sh
physpatch -- "488b?????48"
```

See `physpatch --help` for all the available arguments and their descriptions.

## Installation

### 🦀 Cargo

Using Cargo is the easiest way to get started with PhysPatch:

```sh
cargo install physpatch
sudo setcap "CAP_SYS_PTRACE=ep" $(which physpatch)
```

You can now launch the program with the `physpatch` command.

### 🔩 From Source

```sh
git clone https://github.com/sonodima/physpatch && cd physpatch

cargo build --release
sudo setcap "CAP_SYS_PTRACE=ep" target/release/physpatch
```

The compiled binary is located in `target/release/physpatch`

## Requirements

### ⚠️ THIS TOOL ONLY SUPPORTS X86_64 GUEST SYSTEMS

__CAP_SYS_PTRACE__ is required to use this program without elevation:

```sh
sudo setcap "CAP_SYS_PTRACE=ep" physpatch
```

For more information, refer to the documentation for [memflow_qemu](https://github.com/memflow/memflow-qemu)

## Disclaimer

PhysPatch is an extremely powerful tool, and incorrect usage can *(will)* lead to unintended consequences, including **system crashes** and **data corruption.**

Before using PhysPatch, ensure you fully understand its implications and effects. Proper knowledge of 
the memory structures and patterns you are searching for is essential.

Those associated with PhysPatch will not be held accountable for any damages, data losses, or system
corruptions that arise from the usage of this tool.

## Notable Mentions

This project is heavily inspired by [Hygieia,](https://github.com/Deputation/hygieia) which is a scanning tool to find traces of vulnerable drivers.
