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

pub fn activate_input_gpio(gpio_num: u32) {
    unsafe {
        asm!(
            "sub sp, sp, 16 ",
            "str x1, [sp, 16 * 0]",
            "mov x1, {0:x}",
            "mov x8, 0x83",
            "svc 0x00",
            in(reg) gpio_num,
        );
        asm!(
            "ldr x1, [sp, 16]",
            "add sp, sp, 16",
        );
    }
}

pub fn wait_for_gpio_to_flip(gpio_reg: u32) {
    unsafe {
        asm!(
            "sub sp, sp, 16 ",
            "stp x1, x8, [sp, 16 * 0]",
            "mov x1, {0:x}",
            "mov x8, 0x84",
            "svc 0x00",
            in(reg) gpio_reg,
        );
        asm!(
            "ldp x1, x8, [sp, 16]",
            "add sp, sp, 16",
        );
    }
}

pub fn get_gpio_values(gpio_pins: [bool; 32]) -> [Option<bool>; 32] {
    let mut regs: u32 = 0;
    for i in 0..32 {
        if gpio_pins[i] {
            regs = regs | (1 << i);
        }
    }
    let result: u32;
    unsafe {
        asm!(
            "sub sp, sp, 16 ",
            "stp x1, x8, [sp, 16 * 0]",
            "mov x1, {0:x}",
            "mov x8, 0x85",
            "svc 0x00",
            in(reg) regs,
        );
        asm!(
            "mov {0:x}, x0",
            "ldp x1, x8, [sp, 16]",
            "add sp, sp, 16",
            out(reg) result,
        );
    }

    let mut values = [None; 32];

    for i in 0..32 {
        if gpio_pins[i] {
            values[i] = Some(result & (1 << i) > 0);
        }
    };
    values
}
