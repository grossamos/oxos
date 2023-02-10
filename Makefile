RC := cargo
RUST_SOURCES := $(shell find src -type f -iname '*.rs')
QEMU_ARGS := -M raspi3b -kernel target/aarch64-unknown-none-softfloat/debug/kernel -serial null -chardev stdio,id=uart1 -serial chardev:uart1 -monitor none -qtest unix:/tmp/qtest-gpio.sock

target/aarch64-unknown-none-softfloat/debug/kernel: $(RUST_SOURCES)
	@RUSTFLAGS="-C target-cpu=cortex-a53 -C link-arg=--script=src/linker.ld -C link-arg=--library-path=$(pwd)/src -C target-feature=+strict-align" cargo rustc --target=aarch64-unknown-none-softfloat

qemu: target/aarch64-unknown-none-softfloat/debug/kernel
	qemu-system-aarch64 $(QEMU_ARGS)

qemu-debug: target/aarch64-unknown-none-softfloat/debug/kernel
	qemu-system-aarch64 $(QEMU_ARGS) -S -s
