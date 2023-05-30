.section ".text._start"

.global _start

_start:
    // start mail kernel
    ldr x3, =main
    blr x3


// as a failsafe
halt:
    wfe
    b halt

