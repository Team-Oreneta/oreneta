global load_idt
extern IDT_PTR
load_idt:
    lidt [IDT_PTR]
    ret
