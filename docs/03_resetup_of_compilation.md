# Redoing compilation

## Checking out how the raspi tutorial does it
### Relevant commands
- `RUSTFLAGS="-C target-cpu=cortex-a53 -C link-arg=--library-path=$(pwd)/src/bsp/raspberrypi -C link-arg=--script=kernel.ld -D warnings -D missing_docs" cargo rustc --target=aarch64-unknown-none-softfloat --features bsp_rpi3 --release`
- `rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel kernel8.img`

### Issues faced
- cargo version -> solution (rust-toolchain.toml)

### Modified and working verion
- `RUSTFLAGS="-C target-cpu=cortex-a53 -C link-arg=--script=src/linker.ld -C link-arg=--library-path=$(pwd)/src" cargo rustc --target=aarch64-unknown-none-softfloat`
- `rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel kernel8.img`
