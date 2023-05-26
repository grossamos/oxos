use core::{arch::asm, ptr::{write_bytes, write_volatile}};

use crate::{process::{jump_to_next_program, INSTANCE_STACK_BASE, switch_to_instance_stack}, peripherals::uart_send};

use super::exception_handler::ExceptionContext;

// same number used in linux
const SYSCALL_EXIT: u64         = 0x80;

// The exception context as it is stored on the stack on exception entry.
#[no_mangle]
extern "C" fn execute_syscall(e: &mut ExceptionContext) {
    if e.gpr[8] == SYSCALL_EXIT {
        syscall_exit(e);
    } else {
        panic!("Unknown kernel function");
    }
}

fn syscall_exit(e: &mut ExceptionContext) {
    switch_to_instance_stack();

    // Clean stack of process
    for i in e.sp..INSTANCE_STACK_BASE {
        unsafe {
            write_volatile(i as *mut u32, 0);
        }
    }
    
    // Return from exception
    unsafe {
        asm!( 
            "adr x0, 2f",
            "msr elr_el1, x0",
            "eret",
            "2: nop",
        );
    }

    // jump to next program
    jump_to_next_program();
}

