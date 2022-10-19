.global     _start

_start:     mov r0, $1
            ldr r1, =message
            mov r2, $13
            mov r7, $4

            mov r0, $0
            mov r7, $1


.data
message:    .ascii "Hello World!\n"
