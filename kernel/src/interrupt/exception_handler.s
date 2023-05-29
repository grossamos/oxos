// Macro to save registers (copied from rust embedded)
.macro CALL_WITH_CONTEXT handler
__vector_\handler:
    // registers x0-x18 are caller saved and x29/x30 are the lr and fr
    sub sp, sp, 16 * 3
    stp x19, x20, [sp, 16 * 0]
    str x21, [sp, 16 * 1]

    mov x20, sp
    ldr x19, =KERNEL_STACK_POINTER
    ldr x21, [x19]
    mov sp, x21

    // Make room on the stack for the exception context.
	sub	sp,  sp,  #16 * 18

	// Store all general purpose registers on the stack.
	stp	x0,  x1,  [sp, #16 * 0]
	stp	x2,  x3,  [sp, #16 * 1]
	stp	x4,  x5,  [sp, #16 * 2]
	stp	x6,  x7,  [sp, #16 * 3]
	stp	x8,  x9,  [sp, #16 * 4]
	stp	x10, x11, [sp, #16 * 5]
	stp	x12, x13, [sp, #16 * 6]
	stp	x14, x15, [sp, #16 * 7]
	stp	x16, x17, [sp, #16 * 8]
	stp	x18, x19, [sp, #16 * 9]
	stp	x20, x21, [sp, #16 * 10]
	stp	x22, x23, [sp, #16 * 11]
	stp	x24, x25, [sp, #16 * 12] 
	stp	x26, x27, [sp, #16 * 13]
	stp	x28, x29, [sp, #16 * 14]

	// Add the exception link register (ELR_EL1), saved program status (SPSR_EL1) and exception
	// syndrome register (ESR_EL1).
	mrs	x1,  ELR_EL2
	mrs	x2,  SPSR_EL2
	mrs	x3,  ESR_EL2

	stp	lr,  x1,  [sp, #16 * 15]
	stp	x2,  x3,  [sp, #16 * 16]
    str x20,      [sp, #16 * 17]

	// x0 is the first argument for the function called through `\handler`.
	mov	x0,  sp

	// Call `\handler`.
	bl	\handler

	// After returning from exception handling code, replay the saved context and return via
	// `eret`.
	b	__exception_restore_context

.size	__vector_\handler, . - __vector_\handler
.type	__vector_\handler, function
.endm

.section .text

// base vector has to be aligned to 0x800 (2^11) bits
.align 11

__exception_vector:
.org 0x000
	b unknow_exception
.org 0x080
	b unknow_exception
.org 0x100
	b unknow_exception
.org 0x180
	b unknow_exception

// Current exception level with SP_ELx, x > 0.
.org 0x200
	CALL_WITH_CONTEXT execute_syscall
.org 0x280
	b unknow_exception
.org 0x300
	b unknow_exception
.org 0x380
	b unknow_exception

// Lower exception level, AArch64
.org 0x400
	b unknow_exception
.org 0x480
	b unknow_exception
.org 0x500
	b unknow_exception
.org 0x580
	b unknow_exception

// Lower exception level, AArch32
.org 0x600
	b unknow_exception
.org 0x680
	b unknow_exception
.org 0x700
	b unknow_exception
.org 0x780
	b unknow_exception
.org 0x800


__exception_restore_context:
	ldr	w19,      [sp, #16 * 16]
	ldp	lr,  x20, [sp, #16 * 15]

	msr	SPSR_EL2, x19
	msr	ELR_EL2,  x20

	ldp	x2,  x1,  [sp, #16 * 0]
	ldp	x2,  x3,  [sp, #16 * 1]
	ldp	x4,  x5,  [sp, #16 * 2]
	ldp	x6,  x7,  [sp, #16 * 3]
	ldp	x8,  x9,  [sp, #16 * 4]
	ldp	x10, x11, [sp, #16 * 5]
	ldp	x12, x13, [sp, #16 * 6]
	ldp	x14, x15, [sp, #16 * 7]
	ldp	x16, x17, [sp, #16 * 8]
	ldp	x18, x19, [sp, #16 * 9]
	ldp	x20, x21, [sp, #16 * 10]
	ldp	x22, x23, [sp, #16 * 11]
	ldp	x24, x25, [sp, #16 * 12]
	ldp	x26, x27, [sp, #16 * 13]
	ldp	x28, x29, [sp, #16 * 14]

    mov sp, x20

    ldp x19, x20, [sp, 16 * 0]
    ldr x21, [sp, 16 * 1]
    add sp, sp, 16 * 3

	eret

.size	__exception_restore_context, . - __exception_restore_context
.type	__exception_restore_context, function

