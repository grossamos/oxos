#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(stdsimd)]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::arch::{global_asm, asm};
use framebuffer::Framebuffer;
use gpio::{blink_on, enable_blink, blink_off};
use uart::{uart_init, uart_send};

mod framebuffer;
mod program_loader;
mod gpio;
mod panic;
mod uart;
mod utils;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {

    let message = "Hello World! It'sa me Amosio!\n";

    uart_init();
    uart_send(message);

    let fb = Framebuffer::new();
    fb.draw_str("HELLO WORLD! IT IS I, AMOS...");
    uart_send("More text, did it work?");

    enable_blink();
    for _ in 0..20 {
        // on 
        blink_on();
        for _ in 0..500000 {
            unsafe {
                asm!("nop");
            }
        }

        // off
        blink_off();
        for _ in 0..500000 {
            unsafe {
                asm!("nop");
            }
        }
    }


    loop {
    }
}
