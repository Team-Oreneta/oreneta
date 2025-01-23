// Disable linking to the rust standard library
// This is needed because the standard library relies on system functions.
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use fs::tar::Ramdisk;
use multiboot::information::PAddr;

mod framebuffer;
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

// Define the panic handler function
#[panic_handler]
unsafe fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

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
    text::WRITER.lock().boot_message();
    text::WRITER.lock().boot_message_loaded();

    // Create the initial ramdisk
    let initrd = Ramdisk::new(initrd_address);

    let file = initrd.get_file("./etc/hello.txt").unwrap();
    println!("SIZE: {}, CONTENTS:", file.read_name());
    
    file.write_contents();

    println!("This is a test!");
    // Infinite loop to keep the kernel running
    loop {}
}
