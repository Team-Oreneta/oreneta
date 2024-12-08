#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    base_low: u16,
    sel: u16,
    zero: u8,
    flags: u8,
    base_high: u16,
}

#[repr(C, packed(2))]
struct IdtPtr {
    limit: u16,
    base: u32,
}

const INIT_ENTRY: IdtEntry = IdtEntry {
    base_low: 0,
    sel: 0,
    zero: 0,
    flags: 0,
    base_high: 0,
};

static mut IDT: [IdtEntry; 256] = [ INIT_ENTRY; 256 ];

#[no_mangle]
static mut IDT_PTR: IdtPtr = IdtPtr {
    limit: 0,
    base: 0,
};

fn init_idt_ptr() {
    unsafe {
        IDT_PTR = IdtPtr {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: IDT.as_ptr() as u32,
        };
    }
}

extern "C" {
    fn load_idt();
}

pub fn idt_set_gate(num: u8, base: u32, sel: u16, flags: u8) {
    unsafe {
        IDT[num as usize] = IdtEntry {
            base_low: (base & 0xFFFF) as u16,
            sel: sel,
            zero: 0,
            flags: flags,
            base_high: ((base >> 16) & 0xFFFF) as u16,
        }
    }
}

pub fn init_idt() {
    unsafe {
        init_idt_ptr();
        load_idt();
    }
}
