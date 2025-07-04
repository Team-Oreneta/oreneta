ENTRY(kernel_entry)

SECTIONS
{
    . = 0xC0100000;

    .text ALIGN(4K) : AT(ADDR(.text) - 0xC0000000)
    {
        *(.multiboot2)
        *(.text)
    }

    .rodata ALIGN(4K) : AT(ADDR(.rodata) - 0xC0000000)
    {
        *(.rodata)
    }

    .bss ALIGN(4K) : AT(ADDR(.bss) - 0xC0000000)
    {
        *(.COMMON)
        *(.bss)
        *(.initial_stack)
    }

    .data ALIGN(4K) : AT(ADDR(.data) - 0xC0000000)
    {
        *(.data)
    }

    . = ALIGN(4K);
    bitmap_start = .;
    . = . + 1M;
    bitmap_end = .;

    . = ALIGN(4K);
    heap_start = .;
    . = . + 2M;
    heap_end = .;

    end = .;
}
