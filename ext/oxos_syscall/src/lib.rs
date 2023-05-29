#![no_std]
#![no_main]

mod display;

use core::arch::asm;

pub use display::DisplayBuffer;

pub fn uart_send(message: &str) {
    unsafe {
        asm!(
            "sub sp, sp, 16 * 4",
            "stp x1, x2, [sp, 16 * 0]",
            "stp x3, x8, [sp, 16 * 1]",
            "mov x1, {}",
            "mov x2, {}",
            "mov x8, 0x81",
            "svc 0x00",
            in(reg) message.len(),
            in(reg) message.as_ptr() as u64,
        );
        asm!(
            "ldp x1, x2, [sp, 16 * 0]",
            "ldp x3, x8, [sp, 16 * 1]",
            "add sp, sp, 16 * 4",
        );
    }
}

pub fn exit() {
    unsafe {
        asm!(
            "mov x8, 0x80",
            "svc 0x00"
        );
    }
}
