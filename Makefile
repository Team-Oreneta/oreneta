ARCH := x86
TARGET := $(ARCH)-oreneta
LDFILE := linker.ld
LD := clang -target i686-unknown-none
GRUB_CFG := grub.cfg

ASM_SRC_FILES := $(wildcard src/$(ARCH)/asm/*.asm)
ASM_OBJ_FILES := $(patsubst src/$(ARCH)/asm/%.asm, build/$(ARCH)/asm/%.o, $(ASM_SRC_FILES))


oreneta.iso: kernel.bin
	@mkdir -p build/isofiles/boot/grub
	@cp kernel.bin build/isofiles/boot/
	@cp $(GRUB_CFG) build/isofiles/boot/grub
	@grub-mkrescue -o $@ build/isofiles
	@rm -r build/isofiles


# this links kernel.bin as the assembly object files as deps
kernel.bin: $(ASM_OBJ_FILES)
	@$(LD) -T $(LDFILE) -o $@ $(ASM_OBJ_FILES)


# Tabs need to be tabs in Makefiles!!!
# this is the dependency
build/$(ARCH)/asm/%.o: src/$(ARCH)/asm/%.asm
	@mkdir -p $(shell dirname $@)
	@echo "[ASM] $<"
	@nasm -felf32 $< -o $@
