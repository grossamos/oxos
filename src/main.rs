#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(stdsimd)]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::arch::{global_asm, asm};
use batch_loader::jump_to_program;
use framebuffer::Framebuffer;
use gpio::{blink_on, enable_blink, blink_off};
use uart::{uart_init, uart_send};

mod framebuffer;
mod batch_loader;
mod gpio;
mod panic;
mod uart;
mod utils;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {

    //let message = "Hello World! It'sa me Amosio!\n";

    //uart_init();
    //uart_send(message);
    
    jump_to_program(0);

    //let fb = Framebuffer::new();
    //fb.draw_str("HELLO WORLD! IT IS I, AMOS...");
    //uart_send("More text, did it work?");

    loop {
    }
}
