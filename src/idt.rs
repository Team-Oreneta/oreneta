#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    base_low: u16,  // Lower 16 bits of the handler function address
    sel: u16,       // Kernel segment selector
    zero: u8,       // This must always be zero
    flags: u8,      // Flags
    base_high: u16, // Upper 16 bits of the handler function address
}

#[repr(C, packed(2))]
struct IdtPtr {
    limit: u16, // Limit of the table (size - 1)
    base: u32,  // Base address of the start of the table
}

// Initial IDT entry with all fields set to zero
const INIT_ENTRY: IdtEntry = IdtEntry {
    base_low: 0,
    sel: 0,
    zero: 0,
    flags: 0,
    base_high: 0,
};

// Interrupt Descriptor Table with 256 entries
static mut IDT: [IdtEntry; 256] = [ INIT_ENTRY; 256 ];

// Pointer to the IDT
#[no_mangle]
static mut IDT_PTR: IdtPtr = IdtPtr {
    limit: 0,
    base: 0,
};

fn init_idt_ptr() {
    unsafe {
        IDT_PTR = IdtPtr {
            limit: (core::mem::size_of::<[IdtEntry; 256]>() - 1) as u16,
            base: IDT.as_ptr() as u32, // Base address of the IDT
        };
    }
}

extern "C" {
    fn load_idt(); // External assembly function to load the IDT
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
        load_idt();     // Load the IDT
    }
}
