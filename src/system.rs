// Registers structure to hold CPU state

#[repr(C, packed)]
pub struct Registers {
    pub gs: u32,
    pub fs: u32,
    pub es: u32, 
    pub ds: u32,
    pub edi: u32, 
    pub esi: u32, 
    pub ebp: u32,
    pub esp: u32, 
    pub ebx: u32,
    pub edx: u32,
    pub ecx: u32, 
    pub eax: u32,
    pub int_no: u32,
    pub err_code: u32,
    pub eip: u32,
    pub cs: u32,
    pub eflags: u32,
    pub useresp: u32,
    pub ss: u32,
}