use core::arch::asm;

use modular_bitfield::prelude::*;
use crate::{idt, isrs, memory::VIRTUAL_MEMORY_OFFSET, println, qemu_println};

use super::{MB, pmm::{self, PhysicalMemoryManager}};

const PAGE_SIZE: usize = 4096;
const CR0_PG: u32 = 1 << 31;
const CR4_PSE: u32 = 1 << 4;
const KHEAP_INITIAL_SIZE: usize = 1048576;
const ERR_PRESENT: u32 = 0x1;
const ERR_RW: u32 = 0x2;
const ERR_USER: u32 = 0x4;
const ERR_RESERVED: u32 = 0x8;
const ERR_INST: u32 = 0x10;


#[bitfield(bits = 32)]
#[repr(C, packed)]
pub struct PageDirectoryEntry {
    present: bool,
    rw: bool,
    user: bool,
    w_through: bool,
    cache: bool,
    access: bool,
    reserved: bool,
    page_size: B1,
    global: bool,
    available: B3,
    frame: B20,
}

#[bitfield(bits = 32)]
#[repr(C, packed)]
struct PageTableEntry {
    present: bool,
    rw: bool,
    user: bool,
    reserved: B2,
    accessed: bool,
    dirty: bool,
    reserved2: B2,
    available: B3,
    frame: B20,
}

#[repr(C)]
struct PageTable {
    pub pages: [PageTableEntry; 1024],
}

#[repr(C)]
struct PageDirectory {
    pub tables: [PageDirectoryEntry; 1024],
    pub ref_tables: [*mut PageTable; 1024],
}

unsafe extern "C" {
    unsafe static mut initial_page_directory: *const PageDirectory;
}

struct PagingFlags {
    enabled: bool,
    current_directory: *mut PageDirectory,
    heap_enabled: bool,
}

static mut FREE_MEMORY_AREA: *mut u8 = core::ptr::null_mut();

static mut PAGING_OPTIONS: PagingFlags = PagingFlags {
    enabled: false,
    current_directory: core::ptr::null_mut(),
    heap_enabled: false,
};


pub fn init_paging() {
    unsafe {
        if FREE_MEMORY_AREA.is_null() {
            FREE_MEMORY_AREA = pmm::BITMAP.add(pmm::BITMAP_SIZE);
        }
    }

    unsafe {PAGING_OPTIONS.current_directory = dumb_kmalloc(core::mem::size_of::<PageDirectory>(), true); }
    isrs::register_interrupt_handler(14, page_fault_handler);
    println!("Size: 0x{:x}", core::mem::size_of::<PageDirectory>());
    qemu_println!("Set directory");
    println!("Page directory: 0x{:x}", unsafe {PAGING_OPTIONS.current_directory} as usize);
    unsafe {
        core::intrinsics::volatile_set_memory(
            PAGING_OPTIONS.current_directory, 
            0,
            core::mem::size_of::<PageDirectory>()
        );
    }
    // I'm looping here to mamke sure the page fault is on this line.
    loop {}
    qemu_println!("Zeroed!");

    let mut i = VIRTUAL_MEMORY_OFFSET;
    
    while i < VIRTUAL_MEMORY_OFFSET + (4 * MB) {
        allocate_page(unsafe { PAGING_OPTIONS.current_directory }, i as *mut u8, 0, true, true);
        i += PAGE_SIZE;
    }
    qemu_println!("Allocated stuff!");
    i = VIRTUAL_MEMORY_OFFSET + (4 * MB);

    
    while i < VIRTUAL_MEMORY_OFFSET + (4 * MB) + KHEAP_INITIAL_SIZE {
        allocate_page(unsafe { PAGING_OPTIONS.current_directory }, i as *mut u8, 0, true, true);
    }
    qemu_println!("Allocated more stuff!");


    unsafe {
        switch_page_directory(PAGING_OPTIONS.current_directory, false);
        enable_paging();
    }
    allocate_region(unsafe { PAGING_OPTIONS.current_directory }, 0x0, 0x10000, true, true, true);
    qemu_println!("Done!");

}

fn dumb_kmalloc<T>(size: usize, align: bool) -> *mut T {
    let mut ret = unsafe { FREE_MEMORY_AREA } as *mut T;

    if align && !is_aligned(ret) {
        ret = page_align(ret as *const T) as *mut T;
    }
    unsafe { FREE_MEMORY_AREA = FREE_MEMORY_AREA.add(size) };
    
    ret
}


fn virtual_to_physical<T>(dir: *mut PageDirectory, vaddr: *const T) -> *const T {
    if !unsafe {PAGING_OPTIONS.enabled} {
        return unsafe { vaddr.sub(VIRTUAL_MEMORY_OFFSET) }
    }
    let page_directory_index = page_directory_index(vaddr);
    let page_table_index = page_table_index(vaddr);
    let page_frame_offset = page_frame_index(vaddr);

    if unsafe { &*dir }.ref_tables[page_directory_index].is_null() {
        qemu_println!("virtual_to_physical: page directory entry does not exist.");
        return core::ptr::null();
    }

    let page_table = unsafe { &*(*dir).ref_tables[page_directory_index] };

    if !page_table.pages[page_table_index].present() {
        qemu_println!("virtual_to_physical: page table entry does not exist.");
        return core::ptr::null();
    }

    let frame: u32 = page_table.pages[page_table_index].frame();
    let physaddr = ((frame as usize) << 12) + page_frame_offset;
    
    physaddr as *const T

}

fn allocate_region(dir: *mut PageDirectory, start_va: usize, end_va: usize, identity_map: bool, is_kernel: bool, is_writable: bool) {
    let mut start = start_va & 0xfffff000;
    let end = end_va & 0xfffff000;

    while start <= end {
        if identity_map {
            allocate_page(dir, start as *const u8, start / PAGE_SIZE, is_kernel, is_writable);
        } else {
            allocate_page(dir, start as *const u8, 0, is_kernel, is_writable);
        }
        start += PAGE_SIZE;
    }
}

fn allocate_page<T>(dir: *mut PageDirectory, vaddr: *const T, frame: usize, is_kernel: bool, is_writable: bool) {
    let mut table: *mut PageTable = core::ptr::null_mut();
    if dir.is_null() {
        qemu_println!("Page directory is null!");
        return;
    }

    let page_directory_index = page_directory_index(vaddr);
    let page_table_index = page_table_index(vaddr);

    let mut table = unsafe { &*dir }.ref_tables[page_directory_index];

    if table.is_null() {
        if !unsafe {PAGING_OPTIONS.heap_enabled } {
            table = dumb_kmalloc(core::mem::size_of::<PageTable>(), true);
        } else {
            todo!();
        }
        unsafe { core::intrinsics::volatile_set_memory(table, 0, core::mem::size_of::<PageTable>()); }

        unsafe {
            let t = virtual_to_physical(PAGING_OPTIONS.current_directory, table) as u32;
            (&mut *dir).tables[page_directory_index].set_frame(t >> 12);
            (&mut *dir).tables[page_directory_index].set_present(true);
            (&mut *dir).tables[page_directory_index].set_rw(true);
            (&mut *dir).tables[page_directory_index].set_user(true);
            (&mut *dir).tables[page_directory_index].set_page_size(0);
            (&mut *dir).ref_tables[page_directory_index] = table;
        }
    }

    if !unsafe { &*table }.pages[page_table_index].present() {
        let t =
            if frame != 0 {
                frame
            } else {
                pmm::allocate_block()
            } as u32;
        unsafe { &mut *table }.pages[page_table_index].set_frame(t);
        unsafe { &mut *table }.pages[page_table_index].set_present(true);
        unsafe { &mut *table }.pages[page_table_index].set_rw(true);
        unsafe { &mut *table }.pages[page_table_index].set_user(true);
    }
}

pub unsafe fn switch_page_directory(page_dir: *mut PageDirectory, phys: bool) {
    let t: u32 = if !phys {
        // initial_page_directory is a pointer to the temp directory
        virtual_to_physical(initial_page_directory as *mut _, page_dir as *const u8) as u32
    } else {
        page_dir as u32
    };

    // Inline assembly to set CR3
    asm!(
        "mov cr3, {0}",
        in(reg) t,
        options(nostack, preserves_flags)
    );
}
pub unsafe fn enable_paging() {
    let mut cr0: u32;
    let mut cr4: u32;

    asm!(
        "mov {0:e}, cr4",
        out(reg) cr4,
        options(nostack, preserves_flags)
    );

    cr4 &= !CR4_PSE;

    asm!(
        "mov cr4, {0:e}",
        in(reg) cr4,
        options(nostack, preserves_flags)
    );

    asm!(
        "mov {0:e}, cr0",
        out(reg) cr0,
        options(nostack, preserves_flags)
    );

    cr0 |= CR0_PG;

    asm!(
        "mov cr0, {0:e}",
        in(reg) cr0,
        options(nostack, preserves_flags)
    );

    PAGING_OPTIONS.enabled = true;
}

fn free_page<T>(vaddr: *const T, clear_frame: bool) {
    qemu_println!("Fix me please!");
    todo!();
}

fn page_directory_index<T>(vaddr: *const T) -> usize {
    (vaddr as usize) >> 22
}

fn page_table_index<T>(vaddr: *const T) -> usize {
    ((vaddr as usize) >> 12) & 0x3ff
}

fn page_frame_index<T>(vaddr: *const T) -> usize {
    (vaddr as usize) & 0xfff
}


fn is_aligned<T>(addr: *const T) -> bool {
    (addr as usize % PAGE_SIZE) == 0
}

fn page_align<T>(addr: *const T) -> *const T {
    (((addr as usize) & 0xFFFFF000) + 0x1000) as *const T
}

fn page_fault_handler(reg: crate::system::Registers) {
    let faulting_addr: u32;
    unsafe {
        asm!("mov {}, cr2", out(reg) faulting_addr);
    }

    let (eip, err_code) = (reg.eip, reg.err_code);

    println!("Page fault at address: 0x{:08x}", faulting_addr);
    println!("EIP: 0x{:08x} Error code: 0x{:x}", eip, err_code);

    println!("This error happened when a {}-mode process attempted to {} a page. This page was {}, and the reserved bit was {}. It was {} instruction fetch.",
        if reg.err_code & ERR_USER != 0 { "user" } else { "supervisor" },
        if reg.err_code & ERR_RW != 0 { "write" } else { "read" },
        if reg.err_code & ERR_PRESENT == 0 { "not present" } else { "present" },

        if reg.err_code & ERR_RESERVED != 0 { "set" } else { "not set" },
        if reg.err_code & ERR_INST != 0 { "an" } else { "not an" }
    );
}