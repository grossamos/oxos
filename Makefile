BOARD ?= rpi3
RC := cargo
RUST_SOURCES := $(shell find kernel/src -type f -iname '*.rs' && find kernel/src -type f -iname '*.ld' && find kernel/src -type f -iname '*.s' && echo 'Makefile')
CARGO_OPTIONS := --target=aarch64-unknown-none 
CARGO_CONFIG := --config utils/kernel_config.toml
KERNEL_DEBUG := target/aarch64-unknown-none/debug/kernel
KERNEL_RELEASE := target/aarch64-unknown-none/release/kernel
KERNEL_BIN := kernel8.img
QEMU_ARGS := -M raspi3b -serial null -chardev stdio,id=uart1 -serial chardev:uart1 -monitor none 
QEMU_ARGS := $(QEMU_ARGS) -qtest unix:/tmp/qtest-gpio.sock

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
tic-tac-toe: ext/tic-tac-toe/target/tic-tac-toe

${KERNEL_BIN}: $(RUST_SOURCES) utils/progprep.py ext/hello/target/hello ext/tic-tac-toe/target/tic-tac-toe
	cd kernel; \
	cargo build --release $(CARGO_OPTIONS); \
	cargo objcopy --release $(CARGO_OPTIONS) -- -O binary ../$(KERNEL_BIN)
	utils/progprep.py $(KERNEL_BIN) ./ext/hello/target/hello ./ext/tic-tac-toe/target/tic-tac-toe $(KERNEL_BIN)

${KERNEL_RELEASE}: $(RUST_SOURCES)
	cd kernel; \
	cargo build --release $(CARGO_OPTIONS)

${KERNEL_DEBUG_LINK}: $(RUST_SOURCES)
	cd kernel; \
	cargo objcopy $(CARGO_OPTIONS) --release -- --only-keep-debug ../$(KERNEL_DEBUG_LINK)

objdump: ${KERNEL_RELEASE}
	cd kernel; \
	cargo objdump --release -- --disassemble #--no-show-raw-insn | less

qemu: ${KERNEL_BIN}
	qemu-system-aarch64 $(QEMU_ARGS) -kernel ${KERNEL_BIN}

qemu-debug: ${KERNEL_DEBUG_LINK} $(KERNEL_BIN)
	qemu-system-aarch64 $(QEMU_ARGS) -S -s -kernel ${KERNEL_BIN}

ext/hello/target/hello: ext/hello/src/main.rs
	cd ext/hello; \
	cargo build --release --target=aarch64-unknown-none; \
	cargo objcopy --release --target=aarch64-unknown-none -- -O binary target/hello

ext/tic-tac-toe/target/tic-tac-toe: ext/tic-tac-toe/src/main.rs ext/tic-tac-toe/src/rendering.rs
	cd ext/tic-tac-toe; \
	cargo build --release --target=aarch64-unknown-none; \
	cargo objcopy --release --target=aarch64-unknown-none -- -O binary target/tic-tac-toe

gpio:
	./utils/gpio.py

clean:
	rm -f ./kernel8.img 
	rm -rf ./kernel/target
	rm -f ./kernel8.debug
	rm -rf ./ext/*/target/

flash:
	cp ./kernel8.img /run/media/$(USER)/bootfs/
	umount /run/media/$(USER)/bootfs
