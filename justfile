default: build

# OVMF
OVMF_CODE_FILE := "ovmf/OVMF_CODE.fd"
OVMF_VARS_FILE := "ovmf/OVMF_VARS.fd"

# Kernel
KERNEL_TARGET := "x86_64-unknown-none"
KERNEL_BUILD_PATH := "target/x86_64-unknown-none/release/kernel"

# Loader
LOADER_TARGET := "x86_64-unknown-uefi"
LOADER_BUILD_PATH := "target/x86_64-unknown-uefi/release/loader.efi"

kernel:
    cargo build -p kernel --release --target {{KERNEL_TARGET}}

loader:
    cargo build -p loader --release --target {{LOADER_TARGET}}

build: kernel loader
    mkdir -p esp/efi/boot
    cp {{KERNEL_BUILD_PATH}} esp/kernel.bin
    cp {{LOADER_BUILD_PATH}} esp/efi/boot/bootx64.efi

clean:
    cargo clean

run: build
    qemu-system-x86_64 -enable-kvm -serial stdio \
              -drive if=pflash,format=raw,readonly=on,file={{OVMF_CODE_FILE}} \
              -drive if=pflash,format=raw,readonly=on,file={{OVMF_VARS_FILE}} \
              -drive format=raw,file=fat:rw:esp