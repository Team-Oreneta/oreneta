use core::ptr;
use font8x8::legacy::BASIC_LEGACY;

const FRAMEBUFFER: *mut u32 = 0xA0000 as *mut u32;
const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

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
pub fn print_string(x: usize, y: usize, text: &str, color: u32) {
    let mut cursor_x = x;
    for c in text.chars() {
        draw_char(cursor_x, y, c, color);
        cursor_x += 8; // Move to the next character position
    }
}

pub fn boot_message() {
    print_string(8, 8, "Oreneta Booting Up!", 0xFFFFFF);
}
