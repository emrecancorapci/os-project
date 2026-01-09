# UEFI Hello World

A minimal UEFI application written in Rust that displays "Hello world!" and demonstrates basic UEFI boot capabilities.

I aim to create a basic educational OS.

## Overview

This project creates a bare-metal UEFI application that runs directly on UEFI firmware without an operating system. It uses the `uefi` crate to interface with UEFI services and demonstrates:

- Building a `no_std` Rust application
- UEFI entry point configuration
- Logging to UEFI console
- Running in a QEMU virtual machine with OVMF firmware

## Prerequisites

### Required Software

1. **Rust Toolchain**
   - Install Rust via [rustup](https://rustup.rs/)
   - The project will automatically configure the required targets via `rust-toolchain.toml`

2. **QEMU**
   - Install QEMU for x86_64 emulation:

   ```bash
   # Ubuntu/Debian
   sudo apt install qemu-system-x86

   # Arch Linux
   sudo pacman -S qemu-system-x86

   # Fedora
   sudo dnf install qemu-system-x86
   ```

3. **OVMF Firmware**
   - Download OVMF (Open Virtual Machine Firmware) files:

   ```bash
   # Ubuntu/Debian
   sudo apt install ovmf

   # Arch Linux
   sudo pacman -S edk2-ovmf

   # Fedora
   sudo dnf install edk2-ovmf
   ```

## Project Setup

### 1. Install Rust Targets

The required UEFI targets are specified in `rust-toolchain.toml` and will be installed automatically. If needed, you can manually install them:

```bash
rustup target add x86_64-unknown-uefi
rustup target add aarch64-unknown-uefi
rustup target add i686-unknown-uefi
```

### 2. Set Up OVMF Files

The OVMF firmware files are required to run the UEFI application in QEMU but are excluded from git (see `.gitignore`). You need to copy them to the `ovmf/` directory:

```bash
# Create the ovmf directory
mkdir -p ovmf

# Copy OVMF files from system installation
# The location varies by distribution:

# Ubuntu/Debian
cp /usr/share/OVMF/OVMF_CODE.fd ovmf/
cp /usr/share/OVMF/OVMF_VARS.fd ovmf/

# Arch Linux
cp /usr/share/edk2-ovmf/x64/OVMF_CODE.fd ovmf/
cp /usr/share/edk2-ovmf/x64/OVMF_VARS.fd ovmf/

# Fedora
cp /usr/share/edk2/ovmf/OVMF_CODE.fd ovmf/
cp /usr/share/edk2/ovmf/OVMF_VARS.fd ovmf/
```

**Note:** The exact paths may vary depending on your distribution and OVMF package version. Use `find /usr/share -name "OVMF*"` to locate the files on your system.

### 3. Create ESP Directory

The ESP (EFI System Partition) directory structure is also excluded from git:

```bash
mkdir -p esp/efi/boot
```

## Building and Running

### Build the Project

This compiles the Rust code to a UEFI executable (`.efi` file). `run.sh` already does this for you.

```bash
cargo build --target x86_64-unknown-uefi
```

### Run with QEMU

The easiest way to run the application is using the provided script:

```bash
bash run.sh
```

This script will:

1. Build the UEFI application
2. Copy the built `.efi` file to the ESP directory as `bootx64.efi`
3. Launch QEMU with OVMF firmware and the ESP mounted as a FAT filesystem

### Manual Execution

You can also run the steps manually:

```bash
# Make sure the ESP directory is created
mkdir -p esp/efi/boot

# Build
cargo build --target x86_64-unknown-uefi

# Copy to ESP
cp target/x86_64-unknown-uefi/debug/uefi-hello.efi esp/efi/boot/bootx64.efi

# Run in QEMU
qemu-system-x86_64 -enable-kvm \
  -drive if=pflash,format=raw,readonly=on,file=ovmf/OVMF_CODE.fd \
  -drive if=pflash,format=raw,readonly=on,file=ovmf/OVMF_VARS.fd \
  -drive format=raw,file=fat:rw:esp
```

## Troubleshooting

### QEMU fails to start

- Ensure OVMF files are correctly copied to the `ovmf/` directory
- Verify QEMU is installed: `qemu-system-x86_64 --version`
- Check that KVM is available (or remove `-enable-kvm` flag on non-Linux systems)

### Build errors

- Ensure the correct Rust target is installed: `rustup target list --installed`
- Try cleaning the build: `cargo clean && cargo build --target x86_64-unknown-uefi`

### "Hello world!" doesn't appear

- The message appears in the QEMU console window
- The application waits for 10 seconds before exiting
- Check QEMU's serial output or graphical console
