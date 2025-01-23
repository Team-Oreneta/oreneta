use core::ptr;

pub struct Framebuffer {
    pub base_address: *mut u32, // Pointer to the base address of the linear framebuffer
    pub width: usize,           // Width of the framebuffer in pixels
    pub height: usize,          // Height of the framebuffer in pixels
    pub bg_color: u32,          // Background color
}

impl Framebuffer {
    // Create a new framebuffer instance
    pub fn new(base_address: u32, width: usize, height: usize) -> Self {
        Self {
            base_address: base_address as *mut u32,
            width,
            height,
            bg_color: 0x111111,
        }
    }

    // Draw a pixel at (x, y) with the specified color
    pub fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            unsafe {
                ptr::write_volatile(self.base_address.add(y * self.width + x), color);
            }
        }
    }

    // Draw a rectangle at (x, y) with the specified width, height, and color
    pub fn draw_rectangle(&self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for i in 0..width {
            for j in 0..height {
                self.draw_pixel(x + i, y + j, color);
            }
        }
    }
}