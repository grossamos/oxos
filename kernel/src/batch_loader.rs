use core::arch::asm;

extern "C" {
    static mut __end: u32;
    static mut __start: u32;
}

pub fn jump_to_program(index: u32) {
    //let end = unsafe { &__end } as *const u32 as u32;
    //let start = unsafe { &__start } as *const u32 as u32;
    //let kernel_len = end - start;
    //let jump_in_point: u64 = (end + (32 - kernel_len % 32) % 32 + 2 * 32) as u64;
    let jump_in_point = 0x83000 + 0x1000 * index as u64;

    unsafe {
        asm!("br {}", in(reg) jump_in_point);
    }
}
