use core::arch::asm;

#[inline]
pub unsafe fn outb(port: u16, val: u8) {
    // Write a byte to the specified port
    asm!("outb %al, %dx", in("al") val, in("dx") port, options(att_syntax));
}

#[inline]
pub unsafe fn inb(port: u16) -> u8 {
    // Read a byte from the specified port
    let ret: u8;
    asm!("inb %dx, %al", in("dx") port, out("al") ret, options(att_syntax));
    ret
}

#[inline]
pub unsafe fn inw(port: u16) -> u16 {
    // Read a word from the specified port
    let ret: u16;
    asm!("inw %dx, %ax", in("dx") port, out("ax") ret, options(att_syntax));
    ret
}