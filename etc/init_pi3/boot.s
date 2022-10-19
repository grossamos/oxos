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

    // setup the stack (code starts at 0x8000, we want the stuff below)
    ldr r5, =_start
    mov sp, r5

    // clear out the block starting segment (segment with statically allocated variables)
    ldr r4, =__bss_start
    ldr r5, =__bss_end
    mov r6, $0
    b clear_check
clear:
    ldr r6, [r4]!
    add r4, $1
clear_check:
    cmp r4, r5
    blo clear

    // start mail kernel
    ldr r3, =kernel_main
    blx r3


halt:
    wfe
    b halt
