.section ".text._start"

.global _start

_start:
    // halt all cpus but cpu 0
    mrs x5, MPIDR_EL1
    and x5, x5, #3
    cmp x5, #0
    bne halt

    // setup the stack (code starts at 0x80000, we want the stuff below)
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

.section "other"
