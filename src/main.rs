#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(stdsimd)]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::arch::global_asm;
use framebuffer::Framebuffer;
use uart::{uart_init, uart_send};

mod uart;
mod utils;
mod framebuffer;
mod gpio;
mod panic;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let message = "Hello World! It'sa me Amosio!\n";

    uart_init();
    uart_send(message);

    let fb = Framebuffer::new();
    fb.draw_str("HELLO WORLD! IT IS I, AMOS...");
    uart_send("More text, did it work?");

    loop {
    }
}

