use crate::ports;
use crate::text;
use crate::irq;
use core::fmt::Write;
use crate::system;


const KEYBOARD_US: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
    b'9', b'0', b'-', b'=', 8, b'\t', b'q', b'w', b'e', b'r',
    b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n', 0,
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';',
    b'\'', b'`', 0, b'\\', b'z', b'x', b'c', b'v', b'b', b'n',
    b'm', b',', b'.', b'/', 0, b'*', 0, b' ', 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, b'-', 0, 0, 0, b'+', 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
];



/* Handles the keyboard interrupt */
fn keyboard_handler(r: *const system::Registers) {
    let scancode: u8;
    unsafe {
        scancode = ports::inb(0x60);

        if (scancode & 0x80) != 0 {
            // A key was just released.
        }
        else {
            // A key was pressed. Note that held keys will trigger repeated
            // interrupts. Here, we will just translate the keycode into
            // an ASCII char and print, but this could be changed later.
            write!(text::FB, "{}", KEYBOARD_US[scancode as usize] as char).unwrap();
        }
    }
}
pub fn init_keyboard() {
    irq::install_handler(1, keyboard_handler);
}