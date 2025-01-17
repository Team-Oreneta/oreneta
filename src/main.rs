// Disable linking to the rust standard library
// This is needed because the standard library relies on system functions.
#![no_std]
#![no_main]

//mod gdt
use core::panic::PanicInfo;
use multiboot::information::PAddr;

mod gdt;
mod idt;
mod irq;
mod isrs;
mod keyboard;
mod multiboot_fb;
mod ports;
mod system;
mod text;
mod timer;

// Define the panic handler function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Entry point of the kernel
#[no_mangle]
pub unsafe extern "C" fn kmain(info_ptr: PAddr) -> ! {
    // Initialize the GDT, IDT, ISR, IRQ, timer, and keyboard
    gdt::init_gdt();
    idt::init_idt();
    isrs::init_isrs();
    irq::init_irqs();
    timer::init_timer();
    keyboard::init_keyboard();

    // Use the multiboot information structure
    let multiboot_struct = multiboot_fb::use_multiboot(info_ptr);
    // Get the framebuffer from the multiboot structure
    let fb = multiboot_fb::get_framebuffer(multiboot_struct);
    // Set the default framebuffer for text output
    text::set_default_framebuffer(fb);

    // Display boot messages
    text::FB.boot_message();
    text::FB.boot_message_loaded();

    // Infinite loop to keep the kernel running
    loop {}
}
