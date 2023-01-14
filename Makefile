# TODO: move asembly to rust and let it handle linking
CC := aarch64-elf-gcc
RC := cargo
AS := aarch64-elf-as
RUST_BUILD_FLAGS := -C linker=aarch64-elf-ld
RUST_SOURCES := $(shell find src -type f -iname '*.rs')
QEMU_ARGS := -M raspi3b -kernel target/oxos.elf -nographic -serial null -chardev stdio,id=uart1 -serial chardev:uart1 -monitor none

target/oxos.elf: src/linker.ld target/asm/boot.o target/aarch64-unknown-none/debug/liboxos.rlib
	aarch64-elf-gcc -T src/linker.ld -fdata-sections -o target/oxos.elf -ffreestanding -nostdlib src/boot.s target/aarch64-unknown-none/debug/liboxos.rlib -g

target/asm/boot.o: src/boot.s
	$(AS) -c src/boot.s -o target/asm/boot.o -g

target/aarch64-unknown-none/debug/liboxos.rlib: $(RUST_SOURCES)
	export RUSTFLAGS="$(RUST_BUILD_FLAGS)"
	$(RC) build --target aarch64-unknown-none

qemu: target/oxos.elf
	qemu-system-aarch64 $(QEMU_ARGS)

qemu-debug: target/oxos.elf
	qemu-system-aarch64 $(QEMU_ARGS) -S -s
