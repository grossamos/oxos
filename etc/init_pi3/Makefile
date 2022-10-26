kernel7.img: oxos.elf
	arm-none-eabi-objcopy oxos.elf -O binary kernel7.img -g

boot.o: boot.s
	arm-none-eabi-gcc -mcpu=cortex-a7 -fpic -ffreestanding -c boot.s -o boot.o -g

kernel.o: kernel.c 
	arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -std=gnu99 -c kernel.c -o kernel.o -O2 -Wall -Wextra -g -O0

oxos.elf: linker.ld boot.o kernel/target/arm-none-eabihf/debug/libkernel.rlib
	arm-none-eabi-gcc -T linker.ld -o oxos.elf -ffreestanding -O2 -nostdlib boot.o kernel/target/arm-none-eabihf/debug/libkernel.rlib -z noexecstack

kernel/target/arm-none-eabihf/debug/libkernel.rlib: kernel/src/main.rs
	@cd kernel; xargo build --target arm-none-eabihf

qemu: oxos.elf
	qemu-system-arm -M raspi2b -nographic -kernel oxos.elf -d cpu -s -S
