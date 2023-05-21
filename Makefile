BOARD ?= rpi3
RC := cargo
RUST_SOURCES := $(shell find kernel/src -type f -iname '*.rs' && find kernel/src -type f -iname '*.ld' && find kernel/src -type f -iname '*.s' && echo 'Makefile')
CARGO_OPTIONS := --target=aarch64-unknown-none 
CARGO_CONFIG := --config utils/kernel_config.toml
KERNEL_DEBUG := target/aarch64-unknown-none/debug/kernel
KERNEL_RELEASE := target/aarch64-unknown-none/release/kernel
KERNEL_BIN := kernel8.img
QEMU_ARGS := -M raspi3b -serial null -chardev stdio,id=uart1 -serial chardev:uart1 -monitor none -qtest unix:/tmp/qtest-gpio.sock
KERNEL_DEBUG_LINK := kernel8.debug

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
hello: ext/hello/target/hello

${KERNEL_BIN}: $(RUST_SOURCES) ext/hello/target/hello utils/progprep.py
	cd kernel; \
	cargo objcopy --release $(CARGO_OPTIONS) -- -O binary ../$(KERNEL_BIN)
	utils/progprep.py $(KERNEL_BIN) ./ext/hello/target/hello $(KERNEL_BIN)

${KERNEL_RELEASE}: $(RUST_SOURCES)
	cd kernel; \
	cargo build --release $(CARGO_OPTIONS)

${KERNEL_DEBUG_LINK}: $(RUST_SOURCES)
	cd kernel; \
	cargo objcopy $(CARGO_OPTIONS) --release -- --only-keep-debug $(KERNEL_DEBUG_LINK)

objdump: ${KERNEL_RELEASE}
	cd kernel; \
	cargo objdump --release -- --disassemble #--no-show-raw-insn | less

gpio-sock:
	cd kernel; \
	rm /tmp/qtest-gpio.fifo -f; \
	mkfifo /tmp/qtest-gpio.fifo; \
	cat /tmp/qtest-gpio.fifo | socat - UNIX-LISTEN:/tmp/qtest-gpio.sock & \
	alias set="echo hello world"

qemu: ${KERNEL_BIN} gpio-sock
	qemu-system-aarch64 $(QEMU_ARGS) -kernel ${KERNEL_BIN}

qemu-debug: ${KERNEL_DEBUG_LINK} $(KERNEL_BIN) gpio-sock
	qemu-system-aarch64 $(QEMU_ARGS) -S -s -kernel ${KERNEL_BIN}

ext/hello/target/hello: ext/hello/src/main.rs
	cd ext/hello; \
	cargo objcopy --release --target=aarch64-unknown-none -- -O binary target/hello

# set default pin (usually indicated in cmdline)
PIN=25
set-gpio: 
	printf 'writel 0x%x 0x%x \n' 0x3f200034 $$((1 << ($(PIN) % 32))) > /tmp/qtest-gpio.fifo

clear-gpio:
	printf 'writel 0x%x 0x%x \n' 0x3f200034 $$((1 << ($(PIN) % 32))) > /tmp/qtest-gpio.fifo

clean:
	rm -f ./kernel8.img 
	rm -rf ./kernel/target
	rm -f ./kernel8.debug
	rm -rf ./ext/hello/target/

flash:
	cp ./kernel8.img /run/media/$(USER)/bootfs/
	umount /run/media/$(USER)/bootfs
