// put code in first part of the kernel binary
.section ".text.boot"

.global _start
    .org 0x80000 // skip to address 0x80000

_start:
    // setup the stack (code starts at 0x8000, we want the stuff below)
    ldr x5, =_start
    mov sp, x5

    // clear out the block starting segment (segment with statically allocated variables)
   
    ldr x4, =__bss_start
    ldr x5, =__bss_end
    mov x6, #0
    b clear_check
clear:
    ldr x6, [x4]
    add x4, x4, #1
clear_check:
    cmp x4, x5
    blo clear

    // start mail kernel
    ldr x3, =kernel_main
    blr x3


// as a failsafe
halt:
    wfe
    b halt
