use crate::irq;
use crate::ports;
use crate::system;
use crate::text;
use core::fmt::Write;

static mut timer_ticks: u32 = 0;
const FREQUENCY: u32 = 1000; // 1 Mhz

fn set_frequency(hz: u32) {
    let divisor = 1193180 / hz;
    unsafe {
        ports::outb(0x43, 0x36);             // Set our command byte 0x36
        ports::outb(0x40, (divisor & 0xFF) as u8);   // Set the low byte of divisor
        ports::outb(0x40, (divisor >> 8) as u8);     // Set the high byte of divisor
    }
}


fn timer_handler(r: *const system::Registers) {
    unsafe {
        /* Increment our 'tick count' */
        timer_ticks += 1;

        if (timer_ticks % FREQUENCY == 0) {
            write!(text::FB, "Another second has passed ({} ticks)\n", timer_ticks);
        }
    }
}

pub fn init_timer() {
    set_frequency(FREQUENCY);
    irq::install_handler(0, timer_handler);
}