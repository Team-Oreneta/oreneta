use multiboot::information::{MemoryManagement, Module, Multiboot, PAddr};
use core::{mem, slice};
use crate::text::Framebuffer;

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

// Static instance of Mem
static mut MEM: Mem = Mem;

// Initialize Multiboot
pub fn use_multiboot(info_ptr: PAddr) -> Multiboot<'static, 'static> {
    unsafe { Multiboot::from_ptr(info_ptr, &mut MEM).expect("Header error!") }
}

// Retrieve framebuffer information from Multiboot
pub fn get_framebuffer(
    multiboot_struct: &Multiboot<'static, 'static>,
) -> Framebuffer {
    let s = multiboot_struct
        .framebuffer_table()
        .expect("Framebuffer not found!");
    Framebuffer {
        framebuffer: s.addr as *mut u32,
        width: s.width as usize,
        height: s.height as usize,
        cursor_x: 0,
        cursor_y: 0,
        scroll_y: 0,
        bg_color: 0x111111u32,
    }
}

pub fn get_module(multiboot_struct: &Multiboot<'static, 'static>) -> u32 {
    (*multiboot_struct.modules().unwrap().next().as_mut().unwrap()).start as u32 // as *mut Module as u32
    //  1
}
