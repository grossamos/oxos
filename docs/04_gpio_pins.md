# GPIO pins

## Emulating GPIO pins in Qemu
As seen in the BCM docs:
- all gpio pins have halternative functions (will have to be disabled)
- when changed, a gpio pin causes an interrupt (there are three lines for this)

The website https://cs.unibo.it/~davide.berardi6/post/20201204-1.html states:
- reverse engineered: open unix socket with: `socat - UNIX-LISTEN:/tmp/qtest-gpio.sock`
