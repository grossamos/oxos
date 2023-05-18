use core::arch::asm;

use crate::uart::{uart_send_number, uart_send};

extern "C" {
    static mut __end: u32;
    static mut __start: u32;
}

pub fn jump_to_program(index: u32) {
    // TODO: use index

    let end = unsafe { &__end } as *const u32 as u32;
    let start = unsafe { &__start } as *const u32 as u32;
    let kernel_len = end - start;
    let jump_in_point: u64 = (end + (32 - kernel_len % 32) % 32 + 2 * 32) as u64;

    //uart_send("\n");
    //uart_send("Hello World, this is some filler studd....\n");
    //uart_send("\n");
    //uart_send_number(start);
    //uart_send("\n");
    //uart_send_number(end);
    //uart_send("\n");
    //uart_send_number(kernel_len);
    //uart_send("\n");
    //uart_send_number(jump_in_point as u32);

    unsafe {
        asm!("br {}", in(reg) jump_in_point);
    }
}
