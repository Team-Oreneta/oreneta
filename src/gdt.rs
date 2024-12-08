use core::{mem, ptr};

#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

#[repr(C, packed)]
struct GdtPtr {
    limit: u16,
    base: u32,
}
static GDT: [GdtEntry; 3] = [
    GdtEntry::new(0, 0, 0, 0),
    GdtEntry::new(0, 0xFFFFFFFF, 0x9A, 0xCF),
    GdtEntry::new(0, 0xFFFFFFFF, 0x92, 0xCF),
];

#[no_mangle]
pub static mut GP: GdtPtr = GdtPtr { base: 0, limit: 0 };

extern "C" {
    fn gdt_flush();
}

pub fn init_gdt() {
    unsafe {
        GP.base = GDT.as_ptr() as u32;
        GP.limit = (GDT.len() * mem::size_of::<GdtEntry>() - 1) as u16;
        gdt_flush();
    }
}

impl GdtEntry {
    pub const fn new(base: u32, limit: u32, access: u8, gran: u8) -> Self {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access: access,
            granularity: (((limit >> 16) & 0x0F) as u8) | (gran & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}
