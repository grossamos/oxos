use core::arch::asm;

use crate::uart::{uart_send_number, uart_send};

extern "C" {
    static mut __end: u32;
    static mut __start: u32;
}

pub fn jump_to_program(index: u32) {
    // TODO: use index
    uart_send_number(0x80000);

    let end = unsafe { &__end } as *const u32 as u32;
    let start = unsafe { &__start } as *const u32 as u32;
    let kernel_len = end - start;
    let jump_in_point = end + (32 - kernel_len % 32) % 32;

    uart_send("\n");
    uart_send_number(start);
    uart_send("\n");
    uart_send_number(end);
    uart_send("\n");
    uart_send_number(kernel_len);
    uart_send("\n");
    uart_send_number(jump_in_point);

    unsafe {
        asm!("b __end");
    }
}
