MAGIC    equ 0xE85250D6 ; Multiboot 2 magic
ARCH     equ 0 ; i386
LEN      equ (header_end - header_start) ; Length of the header
CHECKSUM equ 0x100000000 - (MAGIC + ARCH + LEN)

section .multiboot2
align 8
header_start:
    dd MAGIC
    dd ARCH
    dd LEN
    dd CHECKSUM

fb_tag:
    dw 5
    dw 1
    dd fb_tag_end - fb_tag
    dd 1024
    dd 768
    dd 32
fb_tag_end:

; For some reason, it errors if I don't do this.
; I think there needs to be one of these between
; all multiboot 2 tags.
align 8

    dd 0
    dd 8
header_end:

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
