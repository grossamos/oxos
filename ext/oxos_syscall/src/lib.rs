#![no_std]
#![no_main]

mod display;

use core::arch::asm;

pub use display::DisplayBuffer;

pub fn uart_send(message: &str) {
    unsafe {
        let mut addr_save_0: u64;
        let mut addr_save_1: u64;
        let mut addr_save_2: u64;
        asm!(
            "mov {}, x1",
            "mov {}, x2",
            "mov {}, x8",
            "mov x1, {}",
            "mov x2, {}",
            "mov x8, 0x81",
            "svc 0x00",
            out(reg) addr_save_0,
            out(reg) addr_save_1,
            out(reg) addr_save_2,
            in(reg) message.len(),
            in(reg) message.as_ptr() as u64,
        );
        asm!(
            "mov x1, {}",
            "mov x2, {}",
            "mov x8, {}",
            in(reg) addr_save_0,
            in(reg) addr_save_1,
            in(reg) addr_save_2,
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
