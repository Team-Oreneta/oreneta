MBALIGN  equ 1 << 0
MEMINFO  equ 1 << 1
MBFLAGS  equ MBALIGN | MEMINFO
MAGIC    equ 0x1BADB002
CHECKSUM equ -(MAGIC + MBFLAGS)

section .multiboot
align 4
    dd MAGIC
    dd MBFLAGS
    dd CHECKSUM

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

    ; call the kernel
    extern kmain
    call kmain

    ; In case kmain returns:
    cli ; Disable interrupts
.hang:
    hlt       ; Halt until next interrupt
    jmp .hang ; In case of an NMI, loop
.end: