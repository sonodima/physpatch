<h1 align="center">PhysPatch 🩹</h1>

<div align="center">
  <a href="https://github.com/sonodima/physpatch/actions?workflow=CI">
    <img src="https://github.com/sonodima/physpatch/workflows/CI/badge.svg"/>
  </a>
  <img src="https://img.shields.io/badge/license-MIT-blue.svg"/>
</div>

<br>

> PhysPatch performs physical memory scanning and patching of the entire Windows
> Kernel using DMA.
> 
> [memflow](https://github.com/memflow/memflow) is used to access the virtual
> machine's physical memory, and [AOBscan](https://github.com/sonodima/aobscan)
> to perform the multi-threaded memory scanning.

## Usage

- Scan for "48 8b ? ? ? ? ? 48" in the virtual machine named "TargetVM" and write
"48 8b 00 00" in all the matches.

```sh
physpatch -g "TargetVM" -p "48 8b 00 00" -- "48 8b ? ? ? ? ? 48"
```

- Scan for "488b?????48" in the first virtual machine found, without performing
any patching.

```sh
physpatch -- "488b?????48"
```

See `physpatch --help` for all the available arguments and their descriptions.

## Installation

### 🦀 Cargo

Who doesn't love Cargo? You can install **PhysPatch** with it, by running:

```sh
cargo install physpatch
```

You can now launch the program with the `physpatch` command.

### 🔩 From Source

> Rust is required to build from source. If you don't have it installed, you can
> install it using [rustup](https://rustup.rs/).

```sh
git clone https://github.com/sonodima/physpatch
cd physpatch
cargo build --release
```

The compiled binary will be located at `target/release/physpatch`

## Other Information

PhysPatch comes with the [memflow_qemu](https://github.com/memflow/memflow-qemu) and [memflow_win32](https://github.com/memflow/memflow-win32) libraries embedded, so you will not need any setup in your host machine to use it.

__CAP_SYS_PTRACE__ may be required to use this program without root privileges. For more information, refer to [memflow_qemu](https://github.com/memflow/memflow-qemu)'s documentation.

### ⚠️ THIS TOOL ONLY SUPPORTS AMD64 GUEST SYSTEMS

## Notable Mentions

This project is heavily inspired by [Hygieia](https://github.com/Deputation/hygieia), which is a scanning tool to find traces of vulnerable drivers.
