#![no_std]
#![no_main]

use uefi::CStr16;
use uefi::boot::MemoryType;
use uefi::prelude::*;
use uefi::proto::media::file::*;

#[entry]
fn main() -> Status {
    // Initializes global UEFI state used by the `uefi` crate.
    // After this call, we must access firmware services ONLY via `uefi::boot`.
    uefi::helpers::init().unwrap();

    // Handle of the currently running UEFI image (this bootloader).
    let image = boot::image_handle();

    // --- Open filesystem ---
    // Retrieve the Simple File System protocol from the device that loaded us.
    let mut fs = boot::get_image_file_system(image).unwrap();

    // Open the filesystem root directory.
    let mut root = fs.open_volume().unwrap();

    // UEFI file paths are UTF-16 encoded.
    let mut buf = [0u16; 16];
    let kernel_name = CStr16::from_str_with_buf("kernel.bin", &mut buf).unwrap();

    // Open the kernel file in read-only mode.
    let handle = root
        .open(kernel_name, FileMode::Read, FileAttribute::empty())
        .unwrap();

    // Ensure we actually opened a regular file.
    let mut file = match handle.into_type().unwrap() {
        FileType::Regular(f) => f,
        _ => return Status::LOAD_ERROR,
    };

    // --- File metadata ---
    // We need the file size in order to allocate enough RAM.
    let mut info_buf = [0u8; 256];
    let info = file.get_info::<FileInfo>(&mut info_buf).unwrap();
    let size = info.file_size() as usize;

    // Calculate the number of 4 KiB pages required (rounded up).
    let pages = (size + 0xfff) / 0x1000;

    // --- Allocate RAM for the kernel ---
    // UEFI chooses a free physical address and returns it.
    let kernel_addr = uefi::boot::allocate_pages(
        boot::AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        pages,
    )
    .unwrap();

    // Create a mutable slice backed by the allocated physical memory.
    // This is unsafe because Rust cannot verify raw physical addresses.
    let buffer =
        unsafe { core::slice::from_raw_parts_mut(kernel_addr.as_ptr(), size) };

    // Load the kernel image into memory.
    file.read(buffer).unwrap();

    // --- Exit UEFI boot services ---
    // This transfers full control of the system to the kernel.
    // After this call, NO UEFI services may be used.
    let _memory_map =
        unsafe { uefi::boot::exit_boot_services(Some(MemoryType::LOADER_DATA)) };

    // --- Jump to kernel ---
    // Interpret the kernel's load address as an entry function.
    let entry: extern "C" fn() -> ! =
        unsafe { core::mem::transmute(kernel_addr.as_ptr()) };

    // Execution never returns from here.
    entry();
}
