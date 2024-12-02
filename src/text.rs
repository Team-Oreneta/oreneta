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
            // Stop printing if we exceed the screen height
            if CURSOR_Y + 8 > HEIGHT {
                break;
            }

            if c == '\n' {
                CURSOR_X = 0;
                CURSOR_Y += 8;
            } else {
                draw_char(CURSOR_X, CURSOR_Y, c, color);
                CURSOR_X += 8;

                // Wrap to the next line if we exceed the screen width
                if CURSOR_X >= WIDTH {
                    CURSOR_X = 0;
                    CURSOR_Y += 8;
                }
            }

            // Debug: Track cursor position
            // Comment out in release builds
            // println!("CURSOR_X: {}, CURSOR_Y: {}", CURSOR_X, CURSOR_Y);
        }
    }
}

// Print the logo to the framebuffer
pub fn print_logo(color: u32) {
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
        CURSOR_X = 0;
        CURSOR_Y += 8; // Add some spacing if required
    }

    // Print the logo
    print_string(logo, color);
}

fn fill_screen_with_red_dots() {
    const RED_COLOR: u32 = 0xFF0000; // RGB color for red

    for y in (0..HEIGHT).step_by(2) { // Skip alternate rows for dot effect
        for x in (0..WIDTH).step_by(2) { // Skip alternate columns for dot effect
            draw_pixel(x, y, RED_COLOR);
        }
    }
}

// Boot message with the logo
pub fn boot_message() {
    unsafe {
        // print_logo(0xFFFFFF);
        // CURSOR_Y += 8; // Add spacing after the logo
        // print_string(
        //     "Oreneta Booting Up!\nWelcome to Oreneta :D\nMade by Segfault, Poyo, Jake and Elijah with lots of <3.",
        //     0xFFFFFF,
        // );
        fill_screen_with_red_dots();
    }
}
