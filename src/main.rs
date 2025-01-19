// Disable linking to the rust standard library
// This is needed because the standard library relies on system functions.
#![no_std]
#![no_main]

use core::{ffi::c_char, panic::PanicInfo};
use fs::tar::Ramdisk;
use multiboot::information::PAddr;

mod gdt;
mod idt;
mod irq;
mod isrs;
mod keyboard;
mod mb_utils;
mod ports;
mod system;
mod fs;
mod text;
mod timer;
use core::fmt::Write;


// Define the panic handler function
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    write!(text::FB,"{}", info);

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
    let multiboot_struct = mb_utils::use_multiboot(info_ptr);
    // Get the framebuffer from the multiboot structure
    let fb = mb_utils::get_framebuffer(&multiboot_struct);
    // Set the default framebuffer for text output
    text::set_default_framebuffer(fb);
    // Find the address of the first module.
    let initrd_address = mb_utils::get_module(&multiboot_struct);


    // Display boot messages
    text::FB.boot_message();
    text::FB.boot_message_loaded();

    // Create the initial ramdisk
    let initrd = Ramdisk::new(initrd_address);

    let file = initrd.get_file("./etc/hello.txt").unwrap();
    write!(text::FB, "SIZE: {}, CONTENTS:\n", file.read_name());
    
    file.write_contents();

    // Infinite loop to keep the kernel running
    loop {}
}
