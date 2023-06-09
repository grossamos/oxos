# oxos

![oxos demo](./.github/images/oxos.gif)

oxos is a monolithic kernel that can run tic-tac-toe.
Its process model is static and is based around a batch loader.
It can only support one process at a time and the order of programms is set before boot.
oxos has a synchronous interrupt-based Kernel API.
Programms can use kernel functions by interacting with the oxos system library.
The kernel offers syscalls for GPIO, display and UART.

## Getting Started
Building and running oxos requires the following packages to be installed:
- qemu (aarch64)
- cargo & rust (nightly)
- cargo binutils (repo can be found [here](https://github.com/rust-embedded/cargo-binutils))
- python3

Build OXos by running:
```bash
make
```

Launch oxos by running
```bash
make gpio # in a seperate shell (or append "&")
make qemu
```

Debug oxos in gdb 
```bash
make gpio # in a seperate shell (or append "&")
make qemu-debug

# in gdb
add-symbol-file ./kernel8.debug
target remote localhost:1234
break kernel_main
```

## Project Structure

This repository is made up of multiple directories.
`ext` houses all userspace files.
`kernel` hosts the kernel source code.
 `docs` contains some documentation, but is largly out of date at this point.
 `utils` contains helper scripts used in the construction of the binary and emulating gpio.

## References
This project wouldn't be possible without a number of resources.
The most important ones include:
- Andrew Tanenbaum's book on operating systems (see [here](https://csc-knu.github.io/sys-prog/books/Andrew%20S.%20Tanenbaum%20-%20Modern%20Operating%20Systems.pdf))
- Philipp Oppermann's blog (see [here](https://os.phil-opp.com/))
- Sergey Matyukevich's operating system development tutorial (see [here](https://github.com/s-matyukevich/raspberry-pi-os))
