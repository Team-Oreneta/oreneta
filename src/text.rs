use core::ptr;
use font8x8::legacy::BASIC_LEGACY;

const FRAMEBUFFER: *mut u32 = 0xA0000 as *mut u32;
const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

/// A global cursor to keep track of the current print position
static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;

/// Draw a pixel at (x, y) with the specified color
fn draw_pixel(x: usize, y: usize, color: u32) {
    if x < WIDTH && y < HEIGHT {
        unsafe {
            // Write pixel color into the framebuffer
            ptr::write_volatile(FRAMEBUFFER.add(y * WIDTH + x), color);
        }
    }
}

/// Draw a character at (x, y) using the specified color
fn draw_char(x: usize, y: usize, c: char, color: u32) {
    // Directly index into the BASIC_LEGACY font array
    let font = BASIC_LEGACY[c as usize];
    // Iterate over the 8x8 bitmap
    for (row_index, row) in font.iter().enumerate() {
        for col_index in 0..8 {
            if (row >> col_index) & 1 != 0 {
                // Draw pixel if the bit is set
                draw_pixel(x + col_index, y + row_index, color);
            }
        }
    }
}

/// Print a string to the framebuffer
pub fn print_string(text: &str, color: u32) {
    unsafe {
        for c in text.chars() {
            if c == '\n' {
                // Move to the next line
                CURSOR_X = 0;
                CURSOR_Y += 8; // Move 8 pixels down for a new line
            } else {
                draw_char(CURSOR_X, CURSOR_Y, c, color);
                CURSOR_X += 8; // Advance to the next character position
                // Check if we need to wrap to the next line
                if CURSOR_X >= WIDTH {
                    CURSOR_X = 0;
                    CURSOR_Y += 8;
                }
            }
        }
    }
}

/// Boot message with multiple lines
pub fn boot_message() {
    print_string("Oreneta Booting Up!\nWelcome to Oreneta :D\nMade by Segfault, Poyo, Jake and Elijah with lots of <3.", 0xFFFFFF);
}
