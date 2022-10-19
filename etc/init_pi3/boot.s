// put code in first part of the kernel binary
.section ".text.boot"

.global _start
    .org 0x8000 // skip to address 0x8000

_start:
    // halt all other cores
    mrc p15, $0, r5, c0, c0, $5 // get cpu id 
    and r5, $3 // check if cpu id == 3
    cmp r5, $0
    bne halt

    // setup the stack
    // ldr r5, =_start


    mov r1, $0x99
loop:
    add r2, $1
    add r2, $1
    add r2, $1
    b loop


halt:
    wfe
    b halt
