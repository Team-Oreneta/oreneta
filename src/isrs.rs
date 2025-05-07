use core::arch::asm;

use crate::idt;
use crate::println;
use crate::system;

// External ISR function declarations
extern "C" {
    fn isr0();
    fn isr1();
    fn isr2();
    fn isr3();
    fn isr4();
    fn isr5();
    fn isr6();
    fn isr7();
    fn isr8();
    fn isr9();
    fn isr10();
    fn isr11();
    fn isr12();
    fn isr13();
    fn isr14();
    fn isr15();
    fn isr16();
    fn isr17();
    fn isr18();
    fn isr19();
    fn isr20();
    fn isr21();
    fn isr22();
    fn isr23();
    fn isr24();
    fn isr25();
    fn isr26();
    fn isr27();
    fn isr28();
    fn isr29();
    fn isr30();
    fn isr31();
}

// Initialize ISRs
pub fn init_isrs() {
    // Set IDT gates for each ISR
    idt::idt_set_gate(0, isr0 as u32, 0x08, 0x8E);
    idt::idt_set_gate(1, isr1 as u32, 0x08, 0x8E);
    idt::idt_set_gate(2, isr2 as u32, 0x08, 0x8E);
    idt::idt_set_gate(3, isr3 as u32, 0x08, 0x8E);
    idt::idt_set_gate(4, isr4 as u32, 0x08, 0x8E);
    idt::idt_set_gate(5, isr5 as u32, 0x08, 0x8E);
    idt::idt_set_gate(6, isr6 as u32, 0x08, 0x8E);
    idt::idt_set_gate(7, isr7 as u32, 0x08, 0x8E);

    idt::idt_set_gate(8, isr8 as u32, 0x08, 0x8E);
    idt::idt_set_gate(9, isr9 as u32, 0x08, 0x8E);
    idt::idt_set_gate(10, isr10 as u32, 0x08, 0x8E);
    idt::idt_set_gate(11, isr11 as u32, 0x08, 0x8E);
    idt::idt_set_gate(12, isr12 as u32, 0x08, 0x8E);
    idt::idt_set_gate(13, isr13 as u32, 0x08, 0x8E);
    idt::idt_set_gate(14, isr14 as u32, 0x08, 0x8E);
    idt::idt_set_gate(15, isr15 as u32, 0x08, 0x8E);

    idt::idt_set_gate(16, isr16 as u32, 0x08, 0x8E);
    idt::idt_set_gate(17, isr17 as u32, 0x08, 0x8E);
    idt::idt_set_gate(18, isr18 as u32, 0x08, 0x8E);
    idt::idt_set_gate(19, isr19 as u32, 0x08, 0x8E);
    idt::idt_set_gate(20, isr20 as u32, 0x08, 0x8E);
    idt::idt_set_gate(21, isr21 as u32, 0x08, 0x8E);
    idt::idt_set_gate(22, isr22 as u32, 0x08, 0x8E);
    idt::idt_set_gate(23, isr23 as u32, 0x08, 0x8E);

    idt::idt_set_gate(24, isr24 as u32, 0x08, 0x8E);
    idt::idt_set_gate(25, isr25 as u32, 0x08, 0x8E);
    idt::idt_set_gate(26, isr26 as u32, 0x08, 0x8E);
    idt::idt_set_gate(27, isr27 as u32, 0x08, 0x8E);
    idt::idt_set_gate(28, isr28 as u32, 0x08, 0x8E);
    idt::idt_set_gate(29, isr29 as u32, 0x08, 0x8E);
    idt::idt_set_gate(30, isr30 as u32, 0x08, 0x8E);
    idt::idt_set_gate(31, isr31 as u32, 0x08, 0x8E);
}

// Error messages corresponding to ISR numbers
const ERRS: [&str;32] = [
    "Division By Zero",
    "Debug",
    "Non Maskable Interrupt",
    "Breakpoint",
    "Into Detected Overflow",
    "Out of Bounds",
    "Invalid Opcode",
    "No Coprocessor",

    "Double Fault",
    "Coprocessor Segment Overrun",
    "Bad TSS",
    "Segment Not Present",
    "Stack Fault",
    "General Protection Fault",
    "Page Fault",
    "Unknown Interrupt",

    "Coprocessor Fault",
    "Alignment Check",
    "Machine Check",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",

    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved"
];

static mut INTERRUPT_HANDLERS: [fn(system::Registers); 256] = [error_handler; 256];

// Fault handler function
#[no_mangle]
fn fault_handler(r: system::Registers) {
    let int_no = r.int_no;
    let (eax, ebx, ecx, edx,
        esi, edi, ebp, esp,
        eip, err_code) = (r.eax, r.ebx, r.ecx, r.edx,
            r.esi, r.edi, r.ebp, r.esp, r.eip, r.err_code);

    if int_no < 32 {
        unsafe { INTERRUPT_HANDLERS[int_no as usize](r); }
    }

    println!("Registers:");
    println!(" EAX={:08x} EBX={:08x} ECX={:08x} EDX={:08x}", eax, ebx, ecx, edx);
    println!(" ESI={:08x} EDI={:08x} EBP={:08x} ESP={:08x}", esi, edi, ebp, esp);

    unsafe {
        asm!("cli");
        loop {
            asm!("hlt");
        }
    }
}

pub fn register_interrupt_handler(i: usize, f: fn(system::Registers)) {
    unsafe {
        INTERRUPT_HANDLERS[i] = f;
    }
}

fn error_handler(r: system::Registers) {
    let int_no = r.int_no;
    println!("KERNEL PANIC:\nA fatal error occurred and your computer has been halted.\nError code: {}", ERRS[int_no as usize]);
}