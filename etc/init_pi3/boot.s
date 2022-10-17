// put code in first part of the kernel binary
.section ".text.boot"

.global _start
    .org 0x8000 // skip to address 0x8000

_start:
    mov 0x99, %r1
loop:
    mov #0x99, %r2
    b loop
