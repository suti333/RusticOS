# RusticOS

A minimal operating system kernel written in Rust. This project uses **nightly Rust** and builds a **bootable disk image** using `bootimage` and runs on `x86_64` architecture. 
**Note:** This project is heavily inspired by [Phil Opp's *Writing an OS in Rust*](https://os.phil-opp.com/). The blog series provides in-depth tutorials on OS development in Rust and is an excellent learning resource.

## Requirements

- Rust **nightly** (at least `nightly-2020-07-15`)
- `bootimage` tool
- QEMU (for emulation)
- `cargo` and `rustup` installed
- Linux (recommended) or any Unix-like environment

### Install Rust Nightly

```bash
rustup update nightly --force
rustup override set nightly
```

**Note:** The `--force` flag ensures installation even if some components like `rustfmt` are missing.

## Building
To build the kernel:
```bash
cargo build
```

To install `bootimage` and create a bootable disk image:
```bash
cargo install bootimage
cargo bootimage
```

This generates a bootable image at `target/x86_64-rustic_os/debug/bootimage-rustic_os.bin`.

## Running
To run the kernel using QEMU:
```bash
cargo run
```
Ensure `bootimage` and QEMU are installed before running.

## Testing
To run unit and integration tests:
```bash
cargo test
```
This uses a special test configuration to exit QEMU cleanly using `isa-debug-exit`.

## Configuration and Crate Info

Custom build configuration is set in `config.toml`.
This project uses several dependencies, including:
- bootloader with physical memory mapping
- spin, volatile, x86_64 for low-level system programming
- uart_16550, pic8259, pc-keyboard for hardware interfaces
- lazy_static, crossbeam-queue, conquer-once, and futures-util for safe concurrency and async primitives

Full dependency list is available in `Cargo.toml`.
