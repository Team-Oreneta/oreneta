// Disable linking to the rust standard library
// This is needed because the standard library relies on system functions.
#![no_std]
#![no_main]


// mod gdt
use core::panic::PanicInfo;
// mod vga_buffer;
mod text;
// done
// thank you
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// This has to be set to no_mangle because it will be called from assembly.
// Rust by default mangles function and varaible names to include stuff like
// type information. We cannot have that.
// _start is UNIX/ELF callZing convention
// Okay, I asked them to stop having conversations in my thread :)

// lmao sure

#[no_mangle]
pub extern "C" fn kmain() -> ! {
    // vga_buffer::print_something();
    text::printSomething();
    loop {}
}
// Yeah. VGA is an array in memory, each char is two bytes. the char, then a color byte
// is this some kind of "for each character i string print character?"
// feel free to leave silly easter eggs in the comments
// This should work shouldnt it?