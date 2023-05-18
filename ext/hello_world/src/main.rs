#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(stdsimd)]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::{ptr::{read_volatile, write_volatile}, panic::PanicInfo};

const MMIO_BASE:            u32 = 0x3F000000;
const UART_BASE:            u32 = MMIO_BASE + 0x00215000;
const AUX_MU_LSR_REG:       u32 = UART_BASE + 0x54;
const AUX_MU_IO_REG:        u32 = UART_BASE + 0x40;

#[no_mangle]
pub extern "C" fn main() -> ! {
    let test = "AMOS IS HERE! WTF";
    uart_send(test);
    uart_send(test);
    loop {
    }
}

fn uart_send(message: &str) {
    for letter in message.chars() {
        uart_send_letter(letter as u32);
    }
}

fn uart_send_letter(letter: u32) {
    unsafe {
        // wait while FIFO is full
        while read_volatile(AUX_MU_LSR_REG as *const u32) & 0x20 != 0x20 {}
        write_volatile(AUX_MU_IO_REG as *mut u32, letter as u32);
    }

}

#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    uart_send("KERNEL PANIC!");
    loop {}
} 

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
