use core::ptr;
use font8x8::legacy::BASIC_LEGACY;

const LINE_SPACING: usize = 12;

pub struct Framebuffer {
    pub framebuffer: *mut u32,
    pub width: usize,
    pub height: usize,
    pub scroll_y: usize,
    pub cursor_y: usize,
    pub cursor_x: usize,
    pub bg_color: u32,
}

impl Framebuffer {
    // Draw a pixel at (x, y) with the specified color
    fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        // if x < self.width && y < self.height {
        unsafe {
            ptr::write_volatile(self.framebuffer.add(y * self.width + x), color);
        }
        // }
    }

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

    pub fn print_string(&mut self, text: &str, color: u32) {
        self.cursor_x = 0;
        let lines = text.split('\n');
        for line in lines {
            for c in line.chars() {
                // Stop printing if we exceed the screen height
                if self.cursor_y + LINE_SPACING > self.height + self.scroll_y {
                    self.scroll_up(LINE_SPACING, self.bg_color);
                }
    
                self.draw_char(self.cursor_x, self.cursor_y, c, color);
                self.cursor_x += 8;
    
                // Move to the next line if we exceed the screen width
                if self.cursor_x + 8 > self.width {
                    self.cursor_x = 0;
                    self.cursor_y += LINE_SPACING;
                }
            }
            // Move to the next line after finishing the current line
            self.cursor_x = 0;
            self.cursor_y += LINE_SPACING;
        }
    }

    // Print the logo to the framebuffer
    pub fn print_logo(&mut self, color: u32) {
        let logo = r#"
                                                                         .-%@@@*.                   
                                                                          :@@@@@@*                  
                                                                          -@@P@@@@@#                
                                                                        .@@@@O@@@@@@@:              
                                                                         *@@@Y@@@@S@@@              
                                                                         .@@@O@@@@E@@@-             
                                                                          .@@@@@@@GF@@@             
                                                                            *@@@@@@A@@@@.           
        ..+*=.                                                                .@@@@U@@@@@.          
      @@@.  .@@@:                                                      .         *@LT@@@:@          
    *E@+      =@@@.                                                  -@@          .+@@@@@.@.        
   #@L@.       @@@@.    .-..=#=.   .=#*-.     .-..-*=.      :+#=.  .+@@@::.   :=@*%* .@@@@.@.       
   @@@@        :@@@% -@@@@+@@@@. .@@. -@@*.+@@@@@+*@@@+  .@@#  @@@..=@@@....@@* .@@@. .@@@@@%.      
   @@@@        .@@@%  .@@@.  .. -@@@+%@@@@. :@@@.  =@@@. @@@+#@@@@+ :@@@   .::   @@@:   @=.@@@.     
   *@@@-       =@@@   .@@@.     @@@*        .@@@.  .@@@..@@@.       :@@@    .-@@*@@@:    +. #.      
    =@@@.     .@@@.   .@@@.     +@@@.    .  .@@@.  :@@@..@@@@    .. :@@@   *@@@  @@@:     :. -.     
     .+@@@.  *@@.     .@@@+      -@@@@@@@.  %@@@.  #@@@. .@@@@@@@%. .@@@@@=.@@JAKE@@@%     =. ..    
          ..                         .                        .                             =  .-   
                                                                                             -  .:  
                                                                                             .*  
    "#;

        // Print the logo
        self.print_string(logo, color);
        // Reset cursor position after printing the logo
        self.cursor_x = 0;
    }

    fn fill_screen(&self, colors: &[u32]) {
        let num_colors = colors.len();
        let stripe_height = self.height / num_colors;
        for (i, &color) in colors.iter().enumerate() {
            self.draw_rectangle(0, i * stripe_height, self.width, stripe_height, color);
        }
    }

    // Boot message with the logo
    pub fn boot_message(&mut self) {
            self.fill_screen(&[0x050505, 0x111111, 0x121212, 0x222222, 0x232323, 0x333333]);
            self.fill_screen(&[0x111111]);
            self.print_string(
                "Oreneta Booting Up!",
                0xFFFFFF,
            );
            self.print_logo(0xFFFFFF);
            self.print_string(
                "Welcome to Oreneta :D\nMade by Segfault, Poyo, Jake and Elijah with lots of <3.",
                0xFFFFFF,
            );

    }
}
