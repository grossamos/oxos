use core::arch::asm;

extern "C" {
    static mut __end: u32;
    static mut __start: u32;
} 

static mut PROGRAM_COUNTER: u64 = 0;
static mut BASE_STACK_POINTER: u64 = 0;

pub fn jump_to_next_program() {
    let jump_in_point = unsafe {
        PROGRAM_COUNTER += 1;
        0x83000 + 0x1000 * PROGRAM_COUNTER
    };

    unsafe {
        asm!(
            "mov {}, sp", 
            "br {}", 
            out(reg) BASE_STACK_POINTER,
            in(reg) jump_in_point,
        );
    }
}

#[inline(always)]
pub fn clean_stack_from_last_program() {
    unsafe {
        asm!("mov sp, {}", in(reg) BASE_STACK_POINTER);
    }
}
