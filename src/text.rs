use core::ptr;
use font8x8::legacy::BASIC_LEGACY;

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

    // Draw a character at (x, y) using the specified color
    fn draw_char(&self, x: usize, y: usize, c: char, color: u32) {
        let font = BASIC_LEGACY[c as usize];
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
        unsafe {
            for c in text.chars() {
                // Stop printing if we exceed the screen self.height
                if self.cursor_y + 8 > self.height {
                    break;
                }

                if c == '\n' {
                    *&mut self.cursor_x = 0;
                    *&mut self.cursor_y += 8;
                } else {
                    self.draw_char(self.cursor_x, self.cursor_y, c, color);
                    *&mut self.cursor_x += 8;

                    // Wrap to the next line if we exceed the screen self.width
                    if self.cursor_x >= self.width {
                        *&mut self.cursor_x = 0;
                        *&mut self.cursor_y += 8;
                    }
                }

                // Debug: Track cursor position
                // Comment out in release builds
                // println!("&mut self.cursor_x: {}, &mut self.cursor_y: {}", &mut self.cursor_x, &mut self.cursor_y);
            }
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

        // Reset cursor position for logo rendering
        unsafe {
            *&mut self.cursor_x = 0;
            *&mut self.cursor_y += 8; // Add some spacing if required
        }

        // Print the logo
        self.print_string(logo, color);
    }

    fn fill_screen_with_red_dots(&self) {
        const RED_COLOR: u32 = 0xFF0000; // RGB color for red

        for y in (0..self.height) {
            // Skip alternate rows for dot effect
            for x in (0..self.width) {
                // Skip alternate columns for dot effect
                self.draw_pixel(x, y, RED_COLOR);
            }
        }
    }

    // Boot message with the logo
    pub fn boot_message(&self) {
        unsafe {
            // print_logo(0xFFFFFF);
            // &mut self.cursor_y += 8; // Add spacing after the logo
            // self.print_string(
                // "Oreneta Booting Up!\nWelcome to Oreneta :D\nMade by Segfault, Poyo, Jake and Elijah with lots of <3.",
                // 0xFFFFFF,
            // );
            self.fill_screen_with_red_dots();
        }
    }
}
