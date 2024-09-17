
# 🚀 Kernel Wasm

Welcome to the **Kernel Wasm**! This project is a minimal operating system kernel written in Rust, targeting `x86_64` architecture with no standard library (`no_std`). The kernel is built using the Rust nightly toolchain, with WebAssembly runtime capabilities.

## 📋 Prerequisites

Before you begin, ensure you have the following tools installed:

- **Rust (nightly toolchain)**: Install using `rustup`.
- **QEMU**: Emulator to run the kernel image.
- **Cargo bootimage**: Tool to build a bootable image from your kernel.
- **Rust source**: Needed to build the standard library for custom targets.

### Setting up Rust

To install Rust nightly and required components:

```bash
rustup install nightly
rustup default nightly
rustup component add rust-src --toolchain nightly-aarch64-apple-darwin
```

### Installing QEMU

```bash
# macOS (using Homebrew)
brew install qemu
```

## ⚙️ Building the Kernel

First, ensure the `x86_64-unknown-none` target is installed:

```bash
rustup target add x86_64-unknown-none
```

Then, build the kernel and generate a bootable image:

```bash
# Build the kernel
cargo build -Zbuild-std=core,alloc --target x86_64-unknown-none

# Generate the bootable image
cargo bootimage -Zbuild-std=core,alloc
```

Alternatively, you can use the provided `build.sh` script:

```bash
sh ./build.sh
```

## 🚀 Running the Kernel in QEMU

Once the bootable image is generated, you can run it in QEMU:

```bash
qemu-system-x86_64 -drive format=raw,file=target/x86_64-bootloader/debug/bootimage-microkernel.bin
```

## 🛠️ Development Notes

- This project uses the **Rust nightly toolchain** due to the use of unstable features and the need to build the standard library (`core` and `alloc`) for the `x86_64` target.
- The project is built in a **no_std environment**, meaning it does not link to the standard Rust library, and operates with direct hardware interaction.

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.
