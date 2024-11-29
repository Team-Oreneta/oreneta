// Disable linking to the rust standard library
// This is needed because the standard library relies on system functions.
#![no_std]
#![no_main]

use core::panic::PanicInfo;
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

static HELLO: &[u8] = b"Hello, Rust OSDev world!";


#[no_mangle]
pub extern "C" fn kmain() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            // cool! I am listening to my bad membrane keyboard
            *vga_buffer.offset(i as isize * 2) = byte; // dam, this gran turismo playlist im listening to rn is a bangerrrr
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
        // I'm gonna copy my code into this.
    } // oki :D 
    loop {}
}
// Yeah. VGA is an array in memory, each char is two bytes. the char, then a color byte
// is this some kind of "for each character i string print character?"
// feel free to leave silly easter eggs in the comments
// This should work shouldnt it?