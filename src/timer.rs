use core::arch::asm;

use crate::irq;
use crate::ports;
use crate::system;

const FREQUENCY: u32 = 1000; // 1 KHz
static mut TIMER_TICKS: u32 = 0;

fn set_frequency(hz: u32) {
    let divisor = 1193180 / hz;
    unsafe {
        // Set the command byte: 0x36
        ports::outb(0x43, 0x36);
        // Set the low byte of the divisor
        ports::outb(0x40, (divisor & 0xFF) as u8);
        // Set the high byte of the divisor
        ports::outb(0x40, (divisor >> 8) as u8); 
    }
}


fn timer_handler(_r: *const system::Registers) {
    unsafe {
        // Increment the number of timer ticks elapsed since start.
        TIMER_TICKS += 1;
    }
}

pub fn sleepticks(n_ticks: u32) {
    unsafe {
        let start = TIMER_TICKS;
        while TIMER_TICKS < start + n_ticks {
            asm!("hlt");
        }
    }
}

pub fn init_timer() {
    set_frequency(FREQUENCY);
    irq::install_handler(0, timer_handler);
}