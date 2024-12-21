// Disable linking to the rust standard library
// This is needed because the standard library relies on system functions.
#![no_std]
#![no_main]

// mod gdt
use core::panic::PanicInfo;
use multiboot::information::PAddr;

mod multiboot_fb;
mod gdt;
mod idt;
mod isrs;
mod irq;
mod system;
mod ports;
mod timer;
mod text;
mod keyboard;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn kmain(info_ptr: PAddr) -> ! {
    // vga_buffer::print_something();
    gdt::init_gdt();
    idt::init_idt();
    isrs::init_isrs();
    irq::init_irqs();
    timer::init_timer();
    keyboard::init_keyboard();

    let multiboot_struct = multiboot_fb::use_multiboot(info_ptr);
    let fb = multiboot_fb::get_framebuffer(multiboot_struct);
    text::set_default_framebuffer(fb);

    text::FB.boot_message();
    text::FB.boot_message_loaded();

    loop {}
}
