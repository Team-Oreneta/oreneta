use crate::qemu_println;

extern "C" {
    static bitmap_start: u8;
    static bitmap_end: u8;
}


pub struct PhysicalMemoryManager {
    pub bitmap: *mut u8,
    mem_start: *mut u8,
    total_blocks: usize,
    pub bitmap_size: usize,
}

unsafe extern "C" {
    static end: u8;
}

const BLOCK_SIZE: usize = 4096;
const BLOCKS_PER_BUCKET: usize = 8;

#[inline(always)]
fn block_align(addr: usize) -> usize {
    (addr & 0xFFFFF000) + 0x1000
}

pub static mut BITMAP: *mut u8 = core::ptr::null_mut();
static mut MEM_START: *mut u8 = core::ptr::null_mut();
static mut TOTAL_BLOCKS: usize = 0;
pub static mut BITMAP_SIZE: usize = 0;

pub fn init_pmm(mem_size: usize) {
    qemu_println!("PMM: Initializing with {} bytes of memory", mem_size);
    
    let total_blocks = mem_size / BLOCK_SIZE;
    let mut bitmap_size = total_blocks / BLOCKS_PER_BUCKET;

    if bitmap_size * BLOCKS_PER_BUCKET < total_blocks {
        bitmap_size += 1;
    }

    let bitmap = unsafe { &bitmap_start as *const u8 as *mut u8 };
    qemu_println!("PMM: Bitmap location: 0x{:x}", bitmap as usize);

    // Prevent overrun
    let max_bitmap_size = unsafe { (&bitmap_end as *const u8 as usize) - (bitmap as usize) };
    if bitmap_size > max_bitmap_size {
        panic!("Bitmap too large! {} > {}", bitmap_size, max_bitmap_size);
    }

    unsafe {
        qemu_println!("PMM: Zeroing bitmap of size {}", bitmap_size);
        core::intrinsics::volatile_set_memory(bitmap, 0, bitmap_size);
    }

    let mem_start = block_align(bitmap as usize + bitmap_size) as *mut u8;
    qemu_println!("PMM: Memory start: 0x{:x}", mem_start as usize);
    qemu_println!("PMM: Total blocks: {}", total_blocks);

    unsafe {
        BITMAP = bitmap;
        MEM_START = mem_start;
        TOTAL_BLOCKS = total_blocks;
        BITMAP_SIZE = bitmap_size;
    }
}

pub fn allocate_block() -> usize {
    let free_block = first_free_block();
    qemu_println!("PMM: Allocating block {} at physical address 0x{:x}", 
        free_block, free_block * BLOCK_SIZE);
    set_bit(free_block);
    return free_block;
}

fn set_bit(i: usize) {
    let byte_index = i / BLOCKS_PER_BUCKET;
    let bit_offset = i % BLOCKS_PER_BUCKET;
    unsafe {
        let byte_ptr = BITMAP.add(byte_index);

        let current = *byte_ptr;
        *byte_ptr = current | (1 << bit_offset);
    }
}

pub fn first_free_block() -> usize {
    let mut i = 0;
    while i < unsafe { TOTAL_BLOCKS } {
        if !is_set(i) {
            return i;
        }
        i += 1;
    }

    qemu_println!("Ran out of free blocks!");
    // Invalid value (hopefully)
    usize::MAX
}

pub fn is_set(i: usize) -> bool {
    let byte_index = i / BLOCKS_PER_BUCKET;
    let bit_offset = i % BLOCKS_PER_BUCKET;
    unsafe {
        let byte_ptr = BITMAP.add(byte_index);
        ((*byte_ptr >> bit_offset) & 0x1) != 0
    }
}

pub fn test_simple() {
    qemu_println!("Testing PMM...");
    let t1 = first_free_block();
    qemu_println!("First free block: {}", t1);
    let ret = allocate_block();
    qemu_println!("First allocated block PTR: 0x{:x}", ret);
    
    let t2 = first_free_block();
    qemu_println!("Second free block: {}", t2);

    let ret = allocate_block();
    qemu_println!("Second allocated block PTR: 0x{:x}", ret);
}