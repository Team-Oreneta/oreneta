// Disable linking to the rust standard library
// This is needed because the standard library relies on system functions.
#![no_std]
#![no_main]

// mod gdt
use core::{panic::PanicInfo};
use multiboot::information::PAddr;

mod multiboot_fb;
mod text;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kmain(info_ptr: PAddr) -> ! {
    // vga_buffer::print_something();
    let multiboot_struct = multiboot_fb::use_multiboot(info_ptr);
    let mut fb = multiboot_fb::get_framebuffer(multiboot_struct);
    fb.boot_message();
    loop {}
}
