use core::ptr;
use core::fmt;
use font8x8::legacy::BASIC_LEGACY;

const LINE_SPACING: usize = 12;

// Framebuffer static instance
pub static mut FB: Framebuffer = Framebuffer {
    framebuffer: core::ptr::null_mut(),
    width: 0,
    height: 0,
    scroll_y: 0,
    cursor_y: 0,
    cursor_x: 0,
    bg_color: 0,
};

// Framebuffer structure definition
pub struct Framebuffer {
    pub framebuffer: *mut u32,
    pub width: usize,
    pub height: usize,
    pub scroll_y: usize,
    pub cursor_y: usize,
    pub cursor_x: usize,
    pub bg_color: u32,
}

// Set the default framebuffer
pub fn set_default_framebuffer(new_framebuffer: Framebuffer) {
    unsafe {
        FB = new_framebuffer;
    }
}

impl Framebuffer {
    // Draw a pixel at (x, y) with the specified color
    #[inline(always)]
    fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        unsafe {
            ptr::write_volatile(self.framebuffer.add(y * self.width + x), color);
        }
    }

    // Scroll the framebuffer up by a number of lines
    fn scroll_up(&mut self, lines: usize, color: u32) {
        for y in 0..self.height - lines {
            for x in 0..self.width {
                let color = unsafe { ptr::read_volatile(self.framebuffer.add((y + lines) * self.width + x)) };
                self.draw_pixel(x, y, color);
            }
        }

        for y in self.height - lines..self.height {
            for x in 0..self.width {
                self.draw_pixel(x, y, color);
            }
        }
        self.scroll_y += lines;
        self.cursor_y -= lines;
    }

    // Draw a rectangle at (x, y) with the specified width, height, and color
    fn draw_rectangle(&self, x: usize, y: usize, width: usize, height: usize, color: u32) {
        for i in 0..width {
            for j in 0..height {
                self.draw_pixel(x + i, y + j, color);
            }
        }
    }

    // Draw a character at (x, y) using the specified color
    fn draw_char(&self, x: usize, y: usize, c: char, color: u32) {
        let font = BASIC_LEGACY[c as u8 as usize];
        for (row_index, row) in font.iter().enumerate() {
            for col_index in 0..8 {
                if (row >> col_index) & 1 != 0 {
                    self.draw_pixel(x + col_index, y + row_index, color);
                }
            }
        }
    }

    // Print text to the framebuffer
    pub fn print(&mut self, text: &str, color: u32) {
        let lines = text.split('\n');
        for c in text.chars() {
            if self.cursor_y + LINE_SPACING > self.height + self.scroll_y {
                self.scroll_up(LINE_SPACING, self.bg_color);
            }

            self.draw_char(self.cursor_x, self.cursor_y, c, color);
            self.cursor_x += 8;
            if self.cursor_x + 8 > self.width || c == '\n' {
                self.cursor_x = 0;
                self.cursor_y += LINE_SPACING;
            }
        }
    }

    // Print a string to the framebuffer and move to the next line
    pub fn print_string(&mut self, text: &str, color: u32) {
        self.cursor_x = 0;
        self.print(text, color);
        self.cursor_x = 0;
        self.cursor_y += LINE_SPACING;
    }

    // Print the logo to the framebuffer
    pub fn print_logo(&mut self, color: u32) {
        let logo = r#"
                                                                         .-%@@@*.                   
                                                                          :@@@@@@*                  
                                                                          -@@@@@@@@#                
                                                                        .@@@@POYO@@@@:              
                                                                         *@@@@@@@@@@@@              
                                                                         .@@SEGFAULT@@-             
                                                                          .@@@@@@@@@@@@             
                                                                            *@@@JAKE@@@.           
        ..+*=.                                                                .@@@@@@@@@@.          
      @@@.  .@@@:                                                      .         *@ELIJAH@          
    *@@+      =@@@.                                                  -@@          .+@@@@@.@.        
   #@@@.       @@@@.    .-..=#=.   .=#*-.     .-..-*=.      :+#=.  .+@@@::.   :=@*%* .@@@@.@.       
   @@@@        :@@@% -@@@@+@@@@. .@@. -@@*.+@@@@@+*@@@+  .@@#  @@@..=@@@....@@* .@@@. .@@@@@%.      
   @@@@        .@@@%  .@@@.  .. -@@@+%@@@@. :@@@.  =@@@. @@@+#@@@@+ :@@@   .::   @@@:   @=.@@@.     
   *@@@-       =@@@   .@@@.     @@@*        .@@@.  .@@@..@@@.       :@@@    .-@@*@@@:    +. #.      
    =@@@.     .@@@.   .@@@.     +@@@.    .  .@@@.  :@@@..@@@@    .. :@@@   *@@@  @@@:     :. -.     
     .+@@@.  *@@.     .@@@+      -@@@@@@@.  %@@@.  #@@@. .@@@@@@@%. .@@@@@=.@@@@@@@@@%     =. ..    
          ..                         .                        .                             =  .-   
                                                                                             -  .:  
                                                                                             .*    
    "#;

        // Print the logo
        self.print_string(logo, color);
        // Reset cursor position
        self.cursor_x = 0;
    }

    // Fill the screen with stripes of colors
    fn fill_screen(&self, colors: &[u32]) {
        let num_colors = colors.len();
        let stripe_height = self.height / num_colors;
        for (i, &color) in colors.iter().enumerate() {
            self.draw_rectangle(0, i * stripe_height, self.width, stripe_height, color);
        }
    }

    // Display the boot message with the logo
    pub fn boot_message(&mut self) {
        self.fill_screen(&[0x050505, 0x111111, 0x121212, 0x222222, 0x232323, 0x333333]);
        self.fill_screen(&[0x111111]);
        self.print_string(
            "Oreneta Booting Up!",
            0xFFFFFF,
        );
    }

    // Display the boot message after loading
    pub fn boot_message_loaded(&mut self) {
        self.print_logo(0xFFFFFF);
        self.print_string(
            "Welcome to Oreneta :D\nMade by Segfault, Poyo, Jake and Elijah with lots of <3.\n",
            0xFFFFFF,
        );
    }
}

// Implement the fmt::Write trait for Framebuffer
impl fmt::Write for Framebuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s, 0xFFFFFF);
        Ok(())
    }
}