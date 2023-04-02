BOARD ?= rpi3
RC := cargo
RUST_SOURCES := $(shell find src -type f -iname '*.rs' && find src -type f -iname '*.ld' && find src -type f -iname '*.s' && echo 'Makefile')
CARGO_OPTIONS := --target=aarch64-unknown-none
KERNEL_DEBUG := target/aarch64-unknown-none/debug/kernel
KERNEL_RELEASE := target/aarch64-unknown-none/release/kernel
KERNEL_BIN := kernel8.img
QEMU_ARGS := -M raspi3b -serial null -chardev stdio,id=uart1 -serial chardev:uart1 -monitor none -qtest unix:/tmp/qtest-gpio.sock

ifeq ($(BOARD),rpi3)
	CARGO_OPTIONS := $(CARGO_OPTIONS) --features "board_rpi3" --no-default-features
else ifeq ($(BOARD), rpi4)
	CARGO_OPTIONS := $(CARGO_OPTIONS) --features "board_rpi4" --no-default-features
else
	echo "Invalid board name. Valid names include: rpi3, rpi4"
	exit 1
endif

.PHONY: all default release

default: release
all: release
release: $(KERNEL_BIN)

${KERNEL_BIN}: $(KERNEL_RELEASE)
	cargo objcopy --release $(CARGO_OPTIONS) -- -O binary kernel8.img

${KERNEL_RELEASE}: $(RUST_SOURCES)
	cargo build --release $(CARGO_OPTIONS)

${KERNEL_DEBUG}: $(RUST_SOURCES)
	cargo build $(CARGO_OPTIONS)

objdump: ${KERNEL_RELEASE}
	cargo objdump --release -- --disassemble --no-show-raw-insn | less

gpio-sock:
	rm /tmp/qtest-gpio.fifo -f
	mkfifo /tmp/qtest-gpio.fifo
	cat /tmp/qtest-gpio.fifo | socat - UNIX-LISTEN:/tmp/qtest-gpio.sock &
	alias set="echo hello world"

qemu: ${KERNEL_BIN} gpio-sock
	qemu-system-aarch64 $(QEMU_ARGS) -kernel ${KERNEL_BIN}

qemu-debug: ${KERNEL_DEBUG} gpio-sock
	qemu-system-aarch64 $(QEMU_ARGS) -S -s -kernel ${KERNEL_DEBUG}

# set default pin (usually indicated in cmdline)
PIN=25
set-gpio: 
	printf 'writel 0x%x 0x%x \n' 0x3f200034 $$((1 << ($(PIN) % 32))) > /tmp/qtest-gpio.fifo

clear-gpio:
	printf 'writel 0x%x 0x%x \n' 0x3f200034 $$((1 << ($(PIN) % 32))) > /tmp/qtest-gpio.fifo

clean:
	rm -f ./kernel8.img 
	rm -rf ./target

flash:
	cp ./kernel8.img /run/media/$(USER)/bootfs/
	umount /run/media/$(USER)/bootfs
