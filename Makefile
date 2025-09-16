SOURCE:=$(dir $(realpath $(lastword $(MAKEFILE_LIST))))
BUILD?=$(CURDIR)
export RUST_TARGET_PATH=$(SOURCE)/targets

ifeq ($(TARGET),)
	ARCH?=$(shell uname -m)
else
	ARCH?=$(shell echo "$(TARGET)" | cut -d - -f1)
endif

ifeq ($(ARCH),riscv64gc)
	override ARCH:=riscv64
endif
GNU_TARGET=$(ARCH)-unknown-redox


all: $(BUILD)/kernel $(BUILD)/kernel.sym

LD_SCRIPT=$(SOURCE)/linkers/$(ARCH).ld
LOCKFILE=$(SOURCE)/Cargo.lock
MANIFEST=$(SOURCE)/Cargo.toml
TARGET_SPEC=$(RUST_TARGET_PATH)/$(ARCH)-unknown-kernel.json

$(BUILD)/kernel.all: $(LD_SCRIPT) $(LOCKFILE) $(MANIFEST) $(TARGET_SPEC) $(shell find $(SOURCE) -name "*.rs" -type f)
	cargo rustc 
		--bin kernel 
		--manifest-path "$(MANIFEST)" 
		--target "$(TARGET_SPEC)" 
		--release 
		-Z build-std=core,alloc 
		-- 
		-C link-arg=-T -Clink-arg="$(LD_SCRIPT)" 
		-C link-arg=-z -Clink-arg=max-page-size=0x1000 
		--emit link="$(BUILD)/kernel.all"

$(BUILD)/kernel.sym: $(BUILD)/kernel.all
	$(GNU_TARGET)-objcopy 
		--only-keep-debug 
		"$(BUILD)/kernel.all" 
		"$(BUILD)/kernel.sym"

$(BUILD)/kernel: $(BUILD)/kernel.all
	$(GNU_TARGET)-objcopy 
		--strip-debug 
		"$(BUILD)/kernel.all" 
		"$(BUILD)/kernel"

run: all
	mkdir -p isodir/boot/grub
	cp $(BUILD)/kernel isodir/boot/kernel
	echo 'set timeout=0' > isodir/boot/grub/grub.cfg
	echo 'set default=0' >> isodir/boot/grub/grub.cfg
	echo 'menuentry "kernel" {' >> isodir/boot/grub/grub.cfg
	echo '    multiboot2 /boot/kernel' >> isodir/boot/grub/grub.cfg
	echo '    boot' >> isodir/boot/grub/grub.cfg
	echo '}' >> isodir/boot/grub/grub.cfg
	grub-mkrescue -o kernel.iso isodir
	qemu-system-x86_64 -cdrom kernel.iso

clean:
	cargo clean
	rm -f $(BUILD)/kernel $(BUILD)/kernel.sym $(BUILD)/kernel.all kernel.iso
	rm -rf isodir
