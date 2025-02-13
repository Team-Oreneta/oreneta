use crate::framebuffer::Framebuffer;
use multiboot2::{BootInformation, BootInformationHeader};

// Initialize Multiboot
pub fn use_multiboot(info_ptr: usize) -> BootInformation<'static> {
    unsafe { BootInformation::load(info_ptr as *const BootInformationHeader).unwrap() }
}

// Retrieve framebuffer information from Multiboot 2
pub fn get_framebuffer(multiboot_info: &BootInformation<'static>) -> Framebuffer {
    let fb = multiboot_info
        .framebuffer_tag()
        .expect("Framebuffer not found.")
        .unwrap();

    Framebuffer::new(
        fb.address() as u32,
        fb.width() as usize,
        fb.height() as usize,
    )
}

pub fn get_module(multiboot_struct: &BootInformation<'static>) -> u32 {
    let mut modules = multiboot_struct.module_tags();
    modules.next().as_mut().unwrap().start_address() as u32
}
