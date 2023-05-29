use core::arch::{asm, global_asm};

#[repr(C)]
pub struct ExceptionContext {
    // General Purpose Registers.
    pub gpr: [u64; 30],

    // The link register, aka x30.
    pub lr: u64,

    // Exception link register. The program counter at the time the exception happened.
    pub elr_el1: u64,

    // saved program status register
    pub spsr_el1: u64,

    // exception syndrome register
    pub esr_el1: u64,

    // stack pointer
    pub sp: u64,

}

global_asm!(include_str!("./exception_handler.s"));

pub fn init_exception_vector() {
    extern "C" {
        static mut __exception_vector: u64;
    } 

    let stack_el1: u64 = 0x80000 - 0x1000;
    unsafe {
        asm!("msr vbar_el2, {}", in(reg) &__exception_vector as *const u64);
        asm!("mov sp, {}", in(reg) stack_el1);
    }
}

#[no_mangle]
pub extern "C" fn unknow_exception() {
    panic!("Unknown Exception");
}
