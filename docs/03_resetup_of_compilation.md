# Redoing compilation

## Checking out how the raspi tutorial does it
### Relevant commands
- `RUSTFLAGS="-C target-cpu=cortex-a53 -C link-arg=--library-path=$(pwd)/src/bsp/raspberrypi -C link-arg=--script=kernel.ld -D warnings -D missing_docs" cargo rustc --target=aarch64-unknown-none-softfloat --features bsp_rpi3 --release`
- `rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel kernel8.img`
- `qemu-system-aarch64 -M raspi3b -serial stdio -display none -kernel kernel8.img`

### Issues faced
- cargo version -> solution (rust-toolchain.toml)

### Modified and working verion
- `RUSTFLAGS="-C target-cpu=cortex-a53 -C link-arg=--script=src/linker.ld -C link-arg=--library-path=$(pwd)/src" cargo rustc --target=aarch64-unknown-none-softfloat`
- `rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel kernel8.img`
- `qemu-system-aarch64 -M raspi3b -kernel target/oxos.elf -nographic -serial null -chardev stdio,id=uart1 -serial chardev:uart1 -monitor none`

## Minifying raspi tutorials
- Did all that
- result was, the resulting binary worked all along just had to use the correct qemu command!

## Fault
- for line leading to exeption see screenshot
- info about the expection can be read in ESR\_EL, ELR\_EL and FAR\_EL registers (see sc for values)
- from it jumping to line 200 we know its a synchroous exeption in exec. level 2/3 (we have 3)
- for key on how to read see:
    - https://developer.arm.com/documentation/ddi0595/2020-12/AArch64-Registers/ESR-EL1--Exception-Syndrome-Register--EL1-
    - https://developer.arm.com/documentation/100933/0100/Synchronous-and-asynchronous-exceptions
- 0b100101 == Data Abort taken without a change in Exception level
- also Fault on the stage 2 translation (S1PTW)

## More Debuggie
- `STR w13, [x9]` -> store value of w13 into address of x9
- `FAR_EL` = 0x8000000000100108
- `ESR_EL3` = 0x96000040
    - `EC` = first six bits aka 0b100101 => "Data Abort taken without a change in Exception level." causes include MMU faults and data alignment
    - `ISS` (bits 24-5) => WnR is 0b1 meaning abort from reading
