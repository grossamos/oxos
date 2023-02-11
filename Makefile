RC := cargo
RUST_SOURCES := $(shell find src -type f -iname '*.rs')
RUSTFLAGS := -C target-cpu=cortex-a53 -C link-arg=--script=src/linker.ld -C link-arg=--library-path=$(pwd)/src
CARGO_OPTIONS := --target=aarch64-unknown-none-softfloat
KERNEL_DEBUG := target/aarch64-unknown-none-softfloat/debug/kernel
KERNEL_RELEASE := target/aarch64-unknown-none-softfloat/release/kernel
KERNEL_BIN := kernel8.img
QEMU_ARGS := -M raspi3b -serial null -chardev stdio,id=uart1 -serial chardev:uart1 -monitor none -qtest unix:/tmp/qtest-gpio.sock

.PHONY: all default release

default: release
all: release
release: $(KERNEL_RELEASE)

${KERNEL_BIN}: $(KERNEL_RELEASE)
	@RUSTFLAGS="${RUSTFLAGS}" cargo objcopy ${CARGO_OPTIONS} --release -- -O binary kernel8.img --strip-all 

${KERNEL_RELEASE}: $(RUST_SOURCES)
	@RUSTFLAGS="$(RUSTFLAGS)" cargo rustc ${CARGO_OPTIONS} --release

${KERNEL_DEBUG}: $(RUST_SOURCES)
	@RUSTFLAGS="$(RUSTFLAGS)" cargo rustc ${CARGO_OPTIONS}

gpio-sock:
	rm /tmp/qtest-gpio.fifo
	mkfifo /tmp/qtest-gpio.fifo
	cat /tmp/qtest-gpio.fifo | socat - UNIX-LISTEN:/tmp/qtest-gpio.sock &
	alias set="echo hello world"

qemu: ${KERNEL_BIN} gpio-sock
	qemu-system-aarch64 $(QEMU_ARGS) -kernel ${KERNEL_BIN} -s -S

qemu-debug: ${KERNEL_DEBUG} gpio-sock
	qemu-system-aarch64 $(QEMU_ARGS) -S -s -kernel ${KERNEL_DEBUG}

# set default pin (usually indicated in cmdline)
PIN=25
set-gpio: 
	printf 'writel 0x%x 0x%x \n' 0x3f200034 $$((1 << ($(PIN) % 32))) > /tmp/qtest-gpio.fifo

clear-gpio:
	printf 'writel 0x%x 0x%x \n' 0x3f200034 $$((1 << ($(PIN) % 32))) > /tmp/qtest-gpio.fifo
