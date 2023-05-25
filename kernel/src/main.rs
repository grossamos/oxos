#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(stdsimd)]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::arch::{global_asm};
use batch_loader::jump_to_next_program;
use framebuffer::Framebuffer;
use uart::{uart_init, uart_send, uart_send_number};
use exception::{get_exception_level, switch_to_el_1, init_exception_vector};

mod framebuffer;
mod batch_loader;
mod gpio;
mod panic;
mod uart;
mod utils;
mod exception;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let message = "I'm a kernel!\n";
    uart_init();
    uart_send(message);

    let el1 = get_exception_level();
    uart_send_number(el1);

    switch_to_el_1();
    let el2 = get_exception_level();
    uart_send_number(el2);
    uart_send("\n");

    init_exception_vector();

    jump_to_next_program();

    let fb = Framebuffer::new();
    fb.draw_str("HELLO WORLD! IT IS I, AMOS...BLA BLA BLA");

    uart_send("More text, did it work?");
    uart_send_number(42);

    loop {
    }
}
