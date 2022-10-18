// put code in first part of the kernel binary
.section ".text.boot"

.global _start
    .org 0x8000 // skip to address 0x8000

_start:
    mov r1, $0x99
loop:
    add r2, $1
    add r2, $1
    add r2, $1
    b loop
