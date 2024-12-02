use core::ptr;
use font8x8::legacy::BASIC_LEGACY;

const FRAMEBUFFER: *mut u32 = 0xA0000 as *mut u32;
const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

// A global cursor to keep track of the current print position
static mut CURSOR_X: usize = 0;
static mut CURSOR_Y: usize = 0;

// Draw a pixel at (x, y) with the specified color
fn draw_pixel(x: usize, y: usize, color: u32) {
    if x < WIDTH && y < HEIGHT {
        unsafe {
            ptr::write_volatile(FRAMEBUFFER.add(y * WIDTH + x), color);
        }
    }
}

// Draw a character at (x, y) using the specified color
fn draw_char(x: usize, y: usize, c: char, color: u32) {
    let font = BASIC_LEGACY[c as usize];
    for (row_index, row) in font.iter().enumerate() {
        for col_index in 0..8 {
            if (row >> col_index) & 1 != 0 {
                draw_pixel(x + col_index, y + row_index, color);
            }
        }
    }
}

// Print a string to the framebuffer
pub fn print_string(text: &str, color: u32) {
    unsafe {
        for c in text.chars() {
            if c == '\n' {
                CURSOR_X = 0;
                CURSOR_Y += 8;
            } else {
                draw_char(CURSOR_X, CURSOR_Y, c, color);
                CURSOR_X += 8;
                if CURSOR_X >= WIDTH {
                    CURSOR_X = 0;
                    CURSOR_Y += 8;
                }
            }
        }
    }
}

// Print the logo
pub fn print_logo(x: usize, y: usize, color: u32) {
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

    unsafe {
        // Reset cursor to the specified position
        CURSOR_X = x;
        CURSOR_Y = y;

        for line in logo.lines() {
            print_string(line, color);
            CURSOR_X = x;
            CURSOR_Y += 8; // Move down for each line
        }
    }
}

// Boot message with the logo
pub fn boot_message() {
    unsafe {
        print_string(
            "Oreneta Booting Up!\nWelcome to Oreneta :D\nMade by Segfault, Poyo, Jake and Elijah with lots of <3.",
            0xFFFFFF,
        );
        CURSOR_Y += 8; // Add spacing before the logo
        print_logo(0, CURSOR_Y, 0xFFFFFF);
    }
}
