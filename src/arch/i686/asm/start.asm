; %include "config.inc"

; MAGIC     equ 0xE85250D6 ; Multiboot 2 magic
; ARCH      equ 0 ; i386
; LEN       equ (header_end - header_start) ; Length of the header
; CHECKSUM  equ 0x100000000 - (MAGIC + ARCH + LEN)

; VM_BASE   equ 0xC0000000
; PDE_INDEX equ (VM_BASE >> 22)
; PSE_BIT   equ 0x00000010
; PG_BIT    equ 0x80000000


; section .multiboot2
; align 8
; header_start:
;     dd MAGIC
;     dd ARCH
;     dd LEN
;     dd CHECKSUM

; ; We only add this tag if we are using the framebuffer.
; ; Since VGA is default, we just omit it if not.
; %if USING_FB
; fb_tag:
;     dw 5
;     dw 1
;     dd fb_tag_end - fb_tag
;     dd 1024
;     dd 768
;     dd 32
; fb_tag_end:
; %endif

; ; For some reason, it errors if I don't do this.
; ; I think there needs to be one of these between
; ; all multiboot 2 tags.
; align 8

;     dd 0
;     dd 8
; header_end:

; section .data
; align 4096
; global initial_page_directory
; initial_page_directory:
;     dd 0x00000083
;     times(PDE_INDEX - 1) dd 0
;     dd 0x00000083
;     times (1024 - PDE_INDEX - 1) dd 0

; section .initial_stack
; align 32
; stack_bottom:
; resb 104856 ; 1 MB
; stack_top:


; section .text
; ; FIXME: Technically, it works.
; ; Every resource I have read says that the entry point should be
; ; low_kernel_entry, but it doesn't work.
; ; kernel_entry works instead.
; ; I have no idea why this happens, so I want to do more research.
; global low_kernel_entry
; global kernel_entry
; low_kernel_entry equ (kernel_entry - VM_BASE)
; kernel_entry:
;     ; update page directory address, since eax and ebx is in use, have to use ecx or other register
;     mov ecx, (initial_page_directory - VM_BASE)
;     mov cr3, ecx

;     ; Enable 4mb pages
;     mov ecx, cr4;
;     or ecx, PSE_BIT
;     mov cr4, ecx

;     ; Set PG bit, enable paging
;     mov ecx, cr0
;     or ecx, PG_BIT
;     mov cr0, ecx

;     lea ecx, [higher_half]
;     jmp ecx

; higher_half:
;     ; Unmap the first 4mb physical mem, because we don't need it anymore. Flush the tlb too
;     mov dword[initial_page_directory], 0
;     invlpg[0]

;     mov esp, stack_top

;     ; Push the magic number
;     push eax
;     ; Push the multiboot struct
;     push ebx

;     ; call the kernel
;     extern kmain
;     call kmain

;     ; In case kmain returns:
;     cli ; Disable interrupts
; .hang:
;     hlt       ; Halt until next interrupt
;     jmp .hang ; In case of an NMI, loop
; .end:




; %include "config.inc"
%define USING_FB 0

MAGIC     equ 0xE85250D6 ; Multiboot 2 magic
ARCH      equ 0 ; i386
LEN       equ (header_end - header_start)
CHECKSUM  equ 0x100000000 - (MAGIC + ARCH + LEN)

VM_BASE   equ 0xC0000000
PDE_INDEX equ (VM_BASE >> 22)
PSE_BIT   equ 0x00000010
PG_BIT    equ 0x80000000

section .multiboot2
align 8
header_start:
    dd MAGIC
    dd ARCH
    dd LEN
    dd CHECKSUM

%if USING_FB
fb_tag:
    dw 5
    dw 1
    dd fb_tag_end - fb_tag
    dd 1024
    dd 768
    dd 32
fb_tag_end:
%endif

align 8
    dd 0
    dd 8
header_end:

section .data
align 4096
global initial_page_directory
initial_page_directory:
    dd 0x00000083
    times(PDE_INDEX - 1) dd 0
    dd 0x00000083
    times (1024 - PDE_INDEX - 1) dd 0

section .initial_stack
align 32
stack_bottom:
resb 104856 ; 1 MB
stack_top:

section .text
global low_kernel_entry
global kernel_entry
low_kernel_entry equ (kernel_entry - VM_BASE)


; -------------------------------
; Kernel entry point
; -------------------------------
kernel_entry:
    ; update page directory address, since eax and ebx is in use, have to use ecx or other register
    mov ecx, (initial_page_directory - VM_BASE)
    mov cr3, ecx

    ; Enable 4mb pages
    mov ecx, cr4
    or ecx, PSE_BIT
    mov cr4, ecx

    ; Set PG bit, enable paging
    mov ecx, cr0
    or ecx, PG_BIT
    mov cr0, ecx

    lea ecx, [higher_half]
    jmp ecx

higher_half:
    ; Unmap the first 4mb physical mem, because we don't need it anymore. Flush the tlb too
    mov dword[initial_page_directory], 0
    invlpg [0]

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
.end
