global gdt_flush
extern GP

gdt_flush:
    lgdt [GP]
    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    jmp 0x08:far_jump
far_jump:
    ret
