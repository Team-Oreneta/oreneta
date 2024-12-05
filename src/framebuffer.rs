use core::ptr;
use font8x8::legacy::BASIC_LEGACY;

pub struct Framebuffer {
    base_address: *mut u32,
    width: usize,
    height: usize,
    cursor_x: usize,
    cursor_y: usize,
}

impl Framebuffer {
    pub fn new(base_address: usize, width: usize, height: usize) -> Self {
        Self {
            base_address: base_address as *mut u32,
            width,
            height,
            cursor_x: 0,
            cursor_y: 0,
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

    // Draw a character at (x, y) using the specified color
    pub fn draw_char(&self, x: usize, y: usize, c: char, color: u32) {
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
        for c in text.chars() {
            if self.cursor_y + 8 > self.height {
                break;
            }

            if c == '\n' {
                self.cursor_x = 0;
                self.cursor_y += 8;
            } else {
                self.draw_char(self.cursor_x, self.cursor_y, c, color);
                self.cursor_x += 8;

                if self.cursor_x >= self.width {
                    self.cursor_x = 0;
                    self.cursor_y += 8;
                }
            }
        }
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

        self.cursor_x = 0;
        self.cursor_y += 8; // Add some spacing if required
        self.print_string(logo, color);
    }

    // Fill the screen with red dots for a graphical effect
    pub fn fill_screen_with_red_dots(&self) {
        const RED_COLOR: u32 = 0xFF0000; // RGB color for red

        for y in (0..self.height).step_by(2) {
            for x in (0..self.width).step_by(2) {
                self.draw_pixel(x, y, RED_COLOR);
            }
        }
    }

    // Boot message with the logo
    pub fn boot_message(&mut self) {
        self.fill_screen_with_red_dots();
        self.cursor_y += 8; // Add spacing after the dots if needed
        self.print_string(
            "Oreneta Booting Up!\nWelcome to Oreneta :D\nMade by Segfault, Poyo, Jake and Elijah with lots of <3.",
            0xFFFFFF,
        );
    }
}