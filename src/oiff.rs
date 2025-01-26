// Code for reading and parsing the Oreneta Image File Format.
// It is fairly simple:
// a) A magic string that should always equal "OIFF",
// b) A version number for compatibility, and the width and height.
// c) Then, it is padded to 128 bytes for forward compatibility.
// d) After this, the image pixels follow in a 32 bit ARGB format.

#[repr(C, packed)]
pub struct OIFFHeader {
    pub magic: [u8; 4],
    pub version_major: u16,
    pub version_minor: u16,
    pub width: u32,
    pub height: u32,
}


impl OIFFHeader {
    pub unsafe fn parse(data_ptr: *const u32) -> (*const OIFFHeader, &'static [u32]){
        let header = data_ptr as *const OIFFHeader;
        let width = (*header).width as usize;
        let height = (*header).height as usize;

        // We add 32 to the pointer to skip the header and padding.
        let data = unsafe { core::slice::from_raw_parts(data_ptr.add(32), width * height) };

        (header, &data)
    }
}