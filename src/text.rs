use core::ptr;
const FRAMEBUFFER: *mut u32 = 0xA0000 as *mut u32;
const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

fn draw_pixel(x: usize, y: usize, color: u32) {
    unsafe {
        ptr::write_volatile(FRAMEBUFFER.add(y * WIDTH + x), color);
    }
}

fn draw_char(x: usize, y: usize, c: char, color: u32) {
    let font = get_font_data(c);
    font.iter().enumerate().flat_map(|(i, row)| {
        let index = i; 
        row.iter().enumerate().filter_map(move |(j, &pixel)| {
            if pixel != 0 {
                Some((x + j, y + index))
            } else {
                None
            }
        })
    }).for_each(|(px, py)| draw_pixel(px, py, color));
}

fn get_font_data(c: char) -> [[u8; 8]; 8] {
    // Simple 8x8 font data for demonstration purposes
    match c {
        'A' => [
            [0, 1, 1, 1, 1, 1, 1, 0],
            [0, 1, 0, 0, 0, 0, 1, 0],
            [0, 1, 0, 0, 0, 0, 1, 0],
            [0, 1, 1, 1, 1, 1, 1, 0],
            [0, 1, 0, 0, 0, 0, 1, 0],
            [0, 1, 0, 0, 0, 0, 1, 0],
            [0, 1, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ],
        _ => [[0; 8]; 8],
    }
}
pub fn print_something() {
    let text = "A\nA";
    let mut x = 0;
    let mut y = 0;
    for c in text.chars() {
        if c == '\n' {
            y += 8;
            x = 0;
        } else {
            draw_char(x, y, c, 0xFF_FF_FF_FF);
            x += 8;
        }
    }
}