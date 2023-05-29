use core::{arch::asm, ptr::read_volatile};

use crate::peripherals::uart_send;

extern "C" {
    static mut __end: u32;
    static mut __start: u32;
} 

#[no_mangle]
pub static mut KERNEL_STACK_POINTER: u64 = 0;
pub const INSTANCE_STACK_BASE: u64 = 0x70000;
static mut PROGRAM_COUNTER: u64 = 0;

pub fn load_programs() {
    unsafe {
        asm!(
            "mov {}, sp", 
            out(reg) KERNEL_STACK_POINTER,
        );
    }
    jump_to_next_program();

}

pub fn jump_to_next_program() {
    unsafe {
        let jump_in_point = 0x83000 + 0x1000 * PROGRAM_COUNTER;
        PROGRAM_COUNTER += 1;

        if read_volatile(jump_in_point as *const u32) == 0 {
            uart_send("\nFin\n");
            loop {}
        }

        switch_to_instance_stack();

        asm!(
            "br {}", 
            in(reg) jump_in_point,
        );
    }
}

#[inline(always)]
pub fn switch_to_instance_stack() {
    unsafe {
        asm!("mov sp, {}", in(reg) INSTANCE_STACK_BASE);
    }
}
