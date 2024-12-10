use crate::idt;
use crate::ports;
use crate::system;
use core::arch::asm;

extern "C" {
    fn irq0();
    fn irq1();
    fn irq2();
    fn irq3();
    fn irq4();
    fn irq5();
    fn irq6();
    fn irq7();
    fn irq8();
    fn irq9();
    fn irq10();
    fn irq11();
    fn irq12();
    fn irq13();
    fn irq14();
    fn irq15();
}

type IrqFn = fn(*const system::Registers);

static mut IRQ_HANDLERS: [Option<IrqFn>; 16] = [None; 16];

pub fn install_handler(index: usize, function: IrqFn) {
    if index > 15 {
        panic!("Bad error!");
    }
    unsafe {
        IRQ_HANDLERS[index] = Some(function);
    }
}

pub fn uninstall_handler(index: usize) {
    unsafe {
        IRQ_HANDLERS[index] = None;
    }
}

fn remap() {
    unsafe {
        ports::outb(0x20, 0x11);
        ports::outb(0xA0, 0x11);
        ports::outb(0x21, 0x20);
        ports::outb(0xA1, 0x28);
        ports::outb(0x21, 0x04);
        ports::outb(0xA1, 0x02);
        ports::outb(0x21, 0x01);
        ports::outb(0xA1, 0x01);
        ports::outb(0x21, 0x0);
        ports::outb(0xA1, 0x0);
    }
}

pub fn init_irqs() {
    remap();
    idt::idt_set_gate(32, irq0 as u32, 0x08, 0x8E);
    idt::idt_set_gate(33, irq1 as u32, 0x08, 0x8E);
    idt::idt_set_gate(34, irq2 as u32, 0x08, 0x8E);
    idt::idt_set_gate(35, irq3 as u32, 0x08, 0x8E);
    idt::idt_set_gate(36, irq4 as u32, 0x08, 0x8E);
    idt::idt_set_gate(37, irq5 as u32, 0x08, 0x8E);
    idt::idt_set_gate(38, irq6 as u32, 0x08, 0x8E);
    idt::idt_set_gate(39, irq7 as u32, 0x08, 0x8E);
    idt::idt_set_gate(40, irq8 as u32, 0x08, 0x8E);
    idt::idt_set_gate(41, irq9 as u32, 0x08, 0x8E);
    idt::idt_set_gate(42, irq10 as u32, 0x08, 0x8E);
    idt::idt_set_gate(43, irq11 as u32, 0x08, 0x8E);
    idt::idt_set_gate(44, irq12 as u32, 0x08, 0x8E);
    idt::idt_set_gate(45, irq13 as u32, 0x08, 0x8E);
    idt::idt_set_gate(46, irq14 as u32, 0x08, 0x8E);
    idt::idt_set_gate(47, irq15 as u32, 0x08, 0x8E);

    // Enable interrupts
    unsafe {
        asm!("sti");
    }
}

#[no_mangle]
fn irq_handler(r: *const system::Registers) {
    unsafe {
        if let Some(handler) = &IRQ_HANDLERS[((*r).int_no - 32) as usize] {
            handler(r);
        }

        //If the IRQ invoked was IRQ8 - 15, send an EOI to the slave controller
        if (*r).int_no >= 40 {
            ports::outb(0xA0, 0x20);
        }

        // Either way, send an EOI to the master.
        ports::outb(0x20, 0x20);
    }
}
