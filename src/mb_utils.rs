use multiboot::information::{MemoryManagement, Multiboot, PAddr};
use core::{mem, slice};
use crate::framebuffer::Framebuffer;

// Define a struct for memory management
struct Mem;

impl MemoryManagement for Mem {
    // Convert a physical address to a slice
    unsafe fn paddr_to_slice(&self, addr: PAddr, size: usize) -> Option<&'static [u8]> {
        let ptr = mem::transmute(addr as *const u8);
        Some(slice::from_raw_parts(ptr, size))
    }

    unsafe fn allocate(&mut self, _length: usize) -> Option<(PAddr, &mut [u8])> {
        None
    }

    unsafe fn deallocate(&mut self, addr: PAddr) {
        if addr != 0 {
            unimplemented!()
        }
    }
}

// Static instance of Mem for the memory management of multiboot
static mut MEM: Mem = Mem;

// Initialize Multiboot
pub fn use_multiboot(info_ptr: PAddr) -> Multiboot<'static, 'static> {
    unsafe { Multiboot::from_ptr(
        info_ptr, &mut MEM
    ).expect("Header error!") }
}

// Retrieve framebuffer information from Multiboot
pub fn get_framebuffer(
    multiboot_struct: &Multiboot<'static, 'static>,
) -> Framebuffer {
    let s = multiboot_struct
        .framebuffer_table()
        .expect("Framebuffer not found!");

    Framebuffer::new(
        s.addr as u32,
        s.width as usize,
        s.height as usize,
    )
}

pub fn get_module(multiboot_struct: &Multiboot<'static, 'static>) -> u32 {
    let mut modules = multiboot_struct.modules().unwrap();
    modules.next().as_mut().unwrap().start as u32
}
