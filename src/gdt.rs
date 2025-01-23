use core::mem;

//Lots of comments here, this is confusing stuff

// Define a GDT entry structure with specific memory layout
#[repr(C, packed)]
struct GdtEntry {
    limit_low: u16,     // Lower 16 bits of the limit
    base_low: u16,      // Lower 16 bits of the base address
    base_middle: u8,    // Next 8 bits of the base address
    access: u8,         // Access flags
    granularity: u8,    // Granularity and upper 4 bits of the limit
    base_high: u8,      // Highest 8 bits of the base address
}

// Define a GDT pointer structure with specific memory layout
#[repr(C, packed)]
struct GdtPtr {
    limit: u16,         // Size of the GDT
    base: u32,          // Base address of the GDT
}

// Define the GDT with three entries
static GDT: [GdtEntry; 3] = [
    GdtEntry::new(0, 0, 0, 0),                  // Null segment
    GdtEntry::new(0, 0xFFFFFFFF, 0x9A, 0xCF),   // Code segment
    GdtEntry::new(0, 0xFFFFFFFF, 0x92, 0xCF),   // Data segment
];

// Define a mutable static GDT pointer
#[no_mangle]
static mut GP: GdtPtr = GdtPtr { base: 0, limit: 0 };

// Declare an external function to flush the GDT
extern "C" {
    fn gdt_flush();
}

// Initialize the GDT
pub fn init_gdt() {
    unsafe {
        GP.base = GDT.as_ptr() as u32;  // Set the base address of the GDT
        GP.limit = (GDT.len() * mem::size_of::<GdtEntry>() - 1) as u16;  // Set the size of the GDT
        gdt_flush();  // Flush the old GDT and load the new one
    }
}

// Implementation of the GdtEntry struct
impl GdtEntry {
    // Constructor for GdtEntry
    pub const fn new(base: u32, limit: u32, access: u8, gran: u8) -> Self {
        GdtEntry {
            limit_low: (limit & 0xFFFF) as u16,  // Set lower 16 bits of limit
            base_low: (base & 0xFFFF) as u16,    // Set lower 16 bits of base
            base_middle: ((base >> 16) & 0xFF) as u8,  // Set next 8 bits of base
            access: access,  // Set access flags
            granularity: (((limit >> 16) & 0x0F) as u8) | (gran & 0xF0),  // Set granularity and upper 4 bits of limit
            base_high: ((base >> 24) & 0xFF) as u8,  // Set highest 8 bits of base
        }
    }
}
