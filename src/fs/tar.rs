// // Code for the TAR subset of USTAR. Docs can be found here:
// // https://wiki.osdev.org/Tar. 

// // Note that we do NOT have a VFS abstraction. That will be
// // needed later, so TODO.

use core::ffi::CStr;
use core::ffi::c_void;
use crate::print;

#[repr(C, packed)]
pub struct UStarHeader {
    name: [u8; 100],
    mode: [u8; 8],
    uid: [u8; 8],
    gid: [u8; 8],
    size: [u8; 12],
    mtime: [u8; 12],
    checksum: [u8; 8],
    typeflag: u8,
    linkname: [u8; 100],
    magic: [u8; 6],
    version: [u8; 2],
    uname: [u8; 32],
    gname: [u8; 32],
    devmajor: [u8; 8],
    devminor: [u8; 8],
    prefix: [u8; 155],
}

impl UStarHeader {
    fn from_raw(ptr: *const c_void) -> &'static Self {
        unsafe {
            &*(ptr as *const UStarHeader)
        }
    }

    pub fn read_name(&self) -> &str {
        let cstr = unsafe { CStr::from_ptr(self.name.as_ptr() as *const i8) };
        cstr.to_str().unwrap_or("")
    }

    pub fn exists(&self) -> bool {
        let cstr = unsafe { CStr::from_ptr(self.magic.as_ptr() as *const i8) };
        cstr.to_str().unwrap_or("") == "ustar"
    }

    pub fn get_contents_address(&self) -> *const u8 {
        unsafe { (self as *const Self as *const u8).add(512) }
    }

    pub fn read_size(&self) -> usize {
        let mut result: usize = 0;
        for byte in self.size {
            if byte == b'\0' {
                break; // Since this is a C string, it is null terminated.
            }
            let digit = (byte - b'0') as usize;
            if digit > 7 {
                // Invalid octal digit.
                return 0;
            }
            result = result * 8 + digit;
        }
        result
    }

    pub unsafe fn write_contents(&self) {
        let mut i = 0;
        let size = self.read_size();
        let bytes = core::slice::from_raw_parts(self.get_contents_address(), size);
        while i < size {
            print!("{}", bytes[i] as char);
            i += 1;
        }
    }
}

pub unsafe fn read_ustar_header(addr: u32) -> &'static UStarHeader {
    // Here addr is a u32 pointing to the memory location of the header
    let ptr = addr as *const c_void;
    UStarHeader::from_raw(ptr)
}

pub struct Ramdisk {
    address: u32,
}
impl Ramdisk {
    pub fn new(address: u32) -> Self {
        Self { address: address }
    }

    pub unsafe fn get_file(&self, filename: &str) -> Option<&UStarHeader> {
        let mut address = self.address;
        loop {
            // This gets the first header
            let header = read_ustar_header(address as u32);

            if !(header.exists()) {
                break;
            }

            if header.read_name() == filename {
                return Some(header);
            }

            // Find the address of the next header.
            address += (((header.read_size() as u32 + 511) / 512) + 1) * 512;
        }
        // The file was not found.
        None
    }
}