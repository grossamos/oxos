# GPIO pins

## Emulating GPIO pins in Qemu
As seen in the BCM docs:
- all gpio pins have halternative functions (will have to be disabled)
- when changed, a gpio pin causes an interrupt (there are three lines for this)

The website https://cs.unibo.it/~davide.berardi6/post/20201204-1.html states:
- reverse engineered: open unix socket with: `socat - UNIX-LISTEN:/tmp/qtest-gpio.sock`
- script writes to output paths (unsure why..)
- and is able to read stuff where I an manually unable to
- created scripts to try and emulate it

ARM Peripherals: <https://github.com/kshamko/gopherberry/blob/master/docs/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf>
- by default, gpio pins take input as gpio pins and alternative functions are disabled
- we're using the GPLEV0/1 registers to read the actual state of our pins -> bit number coincides with GPIO pin number
- as far as I can read, you have to set the CPU to cause an interrupt when a gpio pin changes 
- base address for gplev0 is `0x 7E20 0034`

Planned tic tac toe:
- indicate row: `21 20 16` == `1 2 3`
- indicate col: `13 6 5` == `1 2 3`
- indicate play: toggle in gpio `25`

## Pi Blink
- Following: <https://www.youtube.com/watch?v=jZT8APrzvc4>
- added config in `.cargo`
- added rust target: `rustup target add armv7a-none-eabi`
- got the dump via: `cargo objdump -- -D target/armv7a-none-eabi/debug/pi_blink`
- doesn't work -> not our uart or framebuffer code is fucked, we still have tooling issues

## OS tutorials
- though: might be an issue with the 3b+
- tried the make files from the tutorials and it worked on both
