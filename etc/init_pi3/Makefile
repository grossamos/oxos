kernel7.img: oxos.elf
	arm-none-eabi-objcopy myos.elf -O binary kernel7.img -g

boot.o: boot.s
	arm-none-eabi-gcc -mcpu=cortex-a7 -fpic -ffreestanding -c boot.s -o boot.o -g

kernel.o: kernel.c 
	arm-none-eabi-gcc -mcpu=arm1176jzf-s -fpic -ffreestanding -std=gnu99 -c kernel.c -o kernel.o -O2 -Wall -Wextra -g

oxos.elf: linker.ld boot.o kernel.o
	arm-none-eabi-gcc -T linker.ld -o oxos.elf -ffreestanding -O2 -nostdlib boot.o kernel.o -lgcc -g

qemu: oxos.elf
	qemu-system-arm -M raspi2b -d in_asm -nographic -kernel oxos.elf -d cpu -s -S
