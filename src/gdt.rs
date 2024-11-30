#[repr(C, packed(2))]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed(2))]
struct GdtPtr {
    limit: u16,
    base: u32,
}
static mut GDT: [GdtEntry; 4] = [
    GdtEntry::new(0, 0, 0, 0),
    GdtEntry::new(0, 0xFFFFF),
    
];
static mut gdt: [GdtPtr; 4] = [NULL, NULL, NULL, NULL];
static mut gp: GdtPtr = GdtPtr {
    limit: 0,
    base: 0,
};
// YES: https://github.com/redox-os/kernel/blob/master/src/arch/x86/gdt.rs