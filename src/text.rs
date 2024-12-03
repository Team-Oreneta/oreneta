use core::ptr;
use font8x8::legacy::BASIC_LEGACY;

const LINE_SPACING: usize = 12;

pub struct Framebuffer {
    pub framebuffer: *mut u32,
    pub width: usize,
    pub height: usize,
    pub cursor_y: usize,
    pub cursor_x: usize,
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

    // Print a string to the framebuffer
    pub fn print_string(&mut self, text: &str, color: u32) {
        self.cursor_x = 0;
            for c in text.chars() {
                // Stop printing if we exceed the screen self.height
                if self.cursor_y + 8 > self.height {
                    break;
                }

                if c == '\n' {
                    *&mut self.cursor_x = 0;
                    *&mut self.cursor_y += LINE_SPACING;
                } else {
                    self.draw_char(self.cursor_x, self.cursor_y, c, color);
                    *&mut self.cursor_x += 8;

                    // Wrap to the next line if we exceed the screen self.width
                    if self.cursor_x >= self.width {
                        *&mut self.cursor_x = 0;
                        *&mut self.cursor_y += LINE_SPACING;
                    }
                }

                // Debug: Track cursor position
                // Comment out in release builds
                // println!("&mut self.cursor_x: {}, &mut self.cursor_y: {}", &mut self.cursor_x, &mut self.cursor_y);
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
