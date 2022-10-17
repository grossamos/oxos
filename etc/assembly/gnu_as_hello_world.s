.global _start

.text

_start:
    mov $1, %rax
    mov $1, %rdi
    mov $message, %rsi
    mov $13, %rdx
    syscall

    mov $60, %rax # exit
    mov $0, %rdi # return code 0
    syscall
    
.data

message:
    .ascii "Hello World!\n"
