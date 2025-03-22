use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use crate::ports;

const COM1: u16 = 0x3f8;

lazy_static! {
    pub static ref SERIAL_WRITER: Mutex<Serial> = Mutex::new(Serial::init_serial_and_new(COM1).unwrap());
}

pub struct Serial {
    port: u16,
}

impl Serial {
    pub fn init_serial_and_new(port: u16) -> Option<Serial> {
        unsafe {
            ports::outb(port + 1, 0x00);
            ports::outb(port + 3, 0x80);
            ports::outb(port + 0, 0x03);
            ports::outb(port + 1, 0x00);
            ports::outb(port + 3, 0x03);
            ports::outb(port + 2, 0xC7);
            ports::outb(port + 4, 0x0B);
            ports::outb(port + 4, 0x1E);
            ports::outb(port + 0, 0xAE);

            let test_byte_response = ports::inb(port + 0);

            if test_byte_response != 0xAE {
                return None
            }

            ports::outb(port + 4, 0x0F);

            Some(Serial { port: port })
        }
    }

    unsafe fn transmission_is_empty(&self) -> bool {
        (ports::inb(self.port + 5) & 0x20) != 0
    }

    pub unsafe fn write_serial(&self, c: char) {
        while !self.transmission_is_empty() {}
        ports::outb(self.port, c as u8);
    }

    pub fn print(&self, s: &str) {
        for c in s.chars() {
            unsafe { self.write_serial(c); }
        }
    }
}

// Implement the fmt::Write trait for Writer
impl fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! qemu_print {
    ($($arg:tt)*) => ($crate::serial::_qemu_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! qemu_println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::qemu_print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _qemu_print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL_WRITER.lock().write_fmt(args).unwrap();
}