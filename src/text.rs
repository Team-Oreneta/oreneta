use core::ptr;
use core::fmt;
use font8x8::legacy::BASIC_LEGACY;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::framebuffer::Framebuffer;
use crate::fs;
use crate::oiff;
use crate::timer::sleepticks;

const LINE_SPACING: usize = 12;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        framebuffer: Framebuffer {
            base_address: core::ptr::null_mut(),
            width: 0,
            height: 0,
            bg_color: 0,
        },
        scroll_y: 0,
        cursor_y: 0,
        cursor_x: 0,
    });
}

unsafe impl Sync for Writer {}
unsafe impl Send for Writer {}
// Framebuffer structure definition
pub struct Writer {
    pub framebuffer: Framebuffer,
    pub scroll_y: usize,
    pub cursor_y: usize,
    pub cursor_x: usize,
}

// Set the default framebuffer
pub fn set_default_framebuffer(new_framebuffer: Framebuffer) {
    WRITER.lock().framebuffer = new_framebuffer;
}

impl Writer {
    // Scroll the framebuffer up by a number of lines
    fn scroll_up(&mut self, lines: usize, color: u32) {
        for y in 0..self.framebuffer.height - lines {
            for x in 0..self.framebuffer.width {
                let color = unsafe { ptr::read_volatile(self.framebuffer.base_address.add((y + lines) * self.framebuffer.width + x)) };
                self.framebuffer.draw_pixel(x, y, color);
            }
        }

        for y in self.framebuffer.height - lines..self.framebuffer.height {
            for x in 0..self.framebuffer.width {
                self.framebuffer.draw_pixel(x, y, color);
            }
        }
        self.scroll_y += lines;
        self.cursor_y -= lines;
    }

    // Draw a character at (x, y) using the specified color
    fn draw_char(&self, x: usize, y: usize, c: char, color: u32) {
        let font = BASIC_LEGACY[c as u8 as usize];
        for (row_index, row) in font.iter().enumerate() {
            for col_index in 0..8 {
                if (row >> col_index) & 1 != 0 {
                    self.framebuffer.draw_pixel(x + col_index, y + row_index, color);
                }
            }
        }
    }

    // Print text to the framebuffer
    pub fn print(&mut self, text: &str, color: u32) {
        for c in text.chars() {
            if self.cursor_y + LINE_SPACING > self.framebuffer.height + self.scroll_y {
                self.scroll_up(LINE_SPACING, self.framebuffer.bg_color);
            }

            self.draw_char(self.cursor_x, self.cursor_y, c, color);
            self.cursor_x += 8;
            if self.cursor_x + 8 > self.framebuffer.width || c == '\n' {
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
        let stripe_height = self.framebuffer.height / num_colors;
        for (i, &color) in colors.iter().enumerate() {
            self.framebuffer.draw_rectangle(0, i * stripe_height, self.framebuffer.width, stripe_height, color);
        }
    }

    // Display the boot message with the logo
    pub unsafe fn boot_message(&mut self, logo_file: &fs::tar::UStarHeader) {
        self.fill_screen(&[0x050505, 0x111111, 0x121212, 0x222222, 0x232323, 0x333333]);
        self.fill_screen(&[0x111111]);
        let (header, contents) = oiff::OIFFHeader::parse(logo_file.get_contents_address() as *const u32);
    
        // let header = contents.as_ptr() as *const oiff::OIFFHeader;
        let width = (*header).width as usize;
        let height = (*header).height as usize;
    
        // println!("WIDTH: {}, HEIGHT: {}", width, height);
    
        let (x, y) = self.framebuffer.get_center_xy(width, height);
        self.framebuffer.draw_image(x, y, width, height, contents);
        sleepticks(2000);
        self.framebuffer.draw_rectangle(x, y, width, height, 0x111111);
        sleepticks(1000);

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

// Implement the fmt::Write trait for Writer
impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s, 0xFFFFFF);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}