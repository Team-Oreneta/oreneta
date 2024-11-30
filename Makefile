ARCH := i686
TARGET := debug

LDFILE := config/linker.ld
LD := clang -target $(ARCH)-elf
GRUB_CFG := config/grub.cfg

# This is where cargo compiles to
LIB_PATH := target/$(ARCH)-oreneta/$(TARGET)/liboreneta.a

ASM_SRC_FILES := $(wildcard src/arch/$(ARCH)/asm/*.asm)
ASM_OBJ_FILES := $(patsubst src/arch/$(ARCH)/asm/%.asm, build/arch/$(ARCH)/asm/%.o, $(ASM_SRC_FILES))

build/oreneta.iso: build/kernel.bin
	@mkdir -p build/isofiles/boot/grub
	@cp build/kernel.bin build/isofiles/boot/
	@cp $(GRUB_CFG) build/isofiles/boot/grub
	@grub-mkrescue -o $@ build/isofiles
	@rm -r build/isofiles

run: build/oreneta.iso
	qemu-system-x86_64 -cdrom $<

# the @ signs at start of lines mean "don't show command"
# This links kernel.bin as the assembly object files as deps
build/kernel.bin: rustbuild $(ASM_OBJ_FILES)
	$(LD) -T $(LDFILE) -o $@ -ffreestanding -nostdlib $(ASM_OBJ_FILES) $(LIB_PATH) -lgcc

rustbuild:
	cargo build

clean:
	rm -rf build
	cargo clean

# Tabs need to be tabs in Makefiles!!!
# maybe we should clean up our config files into a dir?
build/arch/$(ARCH)/asm/%.o: src/arch/$(ARCH)/asm/%.asm
	mkdir -p $(shell dirname $@)
	echo "[ASM] $<"
	nasm -felf32 $< -o $@