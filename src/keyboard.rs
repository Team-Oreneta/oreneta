#[allow(dead_code)]

use crate::irq;
use crate::ports;
use crate::print;
use crate::system;

// Define the US keyboard layout
const KEYBOARD_US: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 8, b'\t', b'q',
    b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', b'\n', 0, b'a', b's', b'd',
    b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0, b'\\', b'z', b'x', b'c', b'v', b'b',
    b'n', b'm', b',', b'.', b'/', 0, b'*', 0, b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    b'-', 0, 0, 0, b'+', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// Define the US keyboard layout with Shift key pressed
const KEYBOARD_US_SHIFTED: [u8; 128] = [
    0, 27, b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 27, b'\t', b'Q',
    b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', b'\n', 0, b'A', b'S', b'D',
    b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', b'~', 0, b'|', b'Z', b'X', b'C', b'V', b'B',
    b'N', b'M', b'<', b'>', b'?', 0, b'*', 0, b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    b'-', 0, 0, 0, b'+', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

// Define function pointers for keys
static mut KEYBOARD_US_FNS: [fn(scancode: u8); 128] = [
    unused, unused, other, other, other, other, other, other, other, other, other, other, other,
    other, unused, other, other, other, other, other, other, other, other, other, other, other,
    other, other, other, ctrl, other, other, other, other, other, other, other, other, other,
    other, other, other, shift, other, other, other, other, other, other, other, other, other,
    other, other, shift, other, alt, other, unused, unused, unused, unused, unused, unused, unused,
    unused, unused, unused, unused, unused, unused, unused, unused, unused, other, unused, unused,
    unused, other, unused, unused, unused, unused, unused, unused, unused, unused, unused, unused,
    unused, unused, unused, unused, unused, unused, unused, unused, unused, unused, unused, unused,
    unused, unused, unused, unused, unused, unused, unused, unused, unused, unused, unused, unused,
    unused, unused, unused, unused, unused, unused, unused, unused, unused, unused, unused, unused,
    unused, unused, unused,
];

// Struct to keep track of keyboard state
struct KeyboardState {
    alt: bool,
    shift: bool,
    ctrl: bool,
}

// Initialize the global keyboard state
static mut STATE: KeyboardState = KeyboardState {
    alt: false,
    shift: false,
    ctrl: false,
};

// Function to print a character to the screen
fn keyboard_putchar(c: char) {
    // When we implement input functions, this is
    // where the code will go.
    print!("{}", c);
}

// Function to handle Alt key press
fn alt(_scancode: u8) {
    unsafe {
        STATE.alt = !STATE.alt;
    }
}

// Function to handle Ctrl key press
fn ctrl(_scancode: u8) {
    unsafe {
        STATE.ctrl = !STATE.ctrl;
    }
}

// Function to handle Shift key press
fn shift(_scancode: u8) {
    unsafe {
        STATE.shift = !STATE.shift;
    }
}

// Function to handle other keys
fn other(scancode: u8) {
    // Any printable key.
    unsafe {
        if (scancode & 0x80) != 0 {
            return;
        } else if KEYBOARD_US[scancode as usize] == 0u8 {
            return;
        } else if STATE.shift {
            keyboard_putchar(KEYBOARD_US_SHIFTED[scancode as usize] as char);
        } else if STATE.ctrl {
            keyboard_putchar('^');
            keyboard_putchar(KEYBOARD_US_SHIFTED[scancode as usize] as char);
        } else {
            keyboard_putchar(KEYBOARD_US[scancode as usize] as char);
        }
    }
}

// Function for unused keys
fn unused(_scancode: u8) {
    // This function is called when a key that is not a character
    // is pressed, but is not mapped to any other function.
}

#[allow(dead_code)]
// Function to map a key to a function
pub fn map_key(scancode: u8, function: fn(u8)) {
    // Maps a key to a function. The function should
    // take the scancode as an argument.

    unsafe {
        KEYBOARD_US_FNS[scancode as usize] = function;
    }
}

// Handles the keyboard interrupt, IRQ 1
fn keyboard_handler(_r: *const system::Registers) {
    let scancode: u8;
    unsafe {
        scancode = ports::inb(0x60);

        // A key was pressed. Note that held keys will trigger repeated
        // interrupts. Here, we will just translate the keycode into
        // an ASCII char and print, but this could be changed later.
        KEYBOARD_US_FNS[(scancode & 0x7F) as usize](scancode);
    }
}

// Initialize the keyboard by installing the handler
pub fn init_keyboard() {
    // IRQ 1
    irq::install_handler(1, keyboard_handler);
}
