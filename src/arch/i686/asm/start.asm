MBALIGN    equ 1 << 0
MEMINFO    equ 1 << 1
VIDEOMODE  equ 1 << 2
MBFLAGS    equ MBALIGN | MEMINFO | VIDEOMODE
MAGIC      equ 0x1BADB002
CHECKSUM   equ -(MAGIC + MBFLAGS)

section .multiboot
align 4
    dd MAGIC
    dd MBFLAGS
    dd CHECKSUM
    dd 0
    dd 0
    dd 0
    dd 0
    dd 0
    
    dd 0
    dd 1024
    dd 768
    dd 32

section .bss
align 16
stack_bottom:
resb 16384
stack_top:


section .text
global _start:function (_start.end - _start)
_start:
    ; Initialise the stack pointer
    mov esp, stack_top

    ; Push the magic number
    push eax
    ; Push the multiboot struct
    push ebx

    ; call the kernel
    extern kmain
    call kmain

    ; In case kmain returns:
    cli ; Disable interrupts
.hang:
    hlt       ; Halt until next interrupt
    jmp .hang ; In case of an NMI, loop
.end: