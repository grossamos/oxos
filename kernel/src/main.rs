#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(stdsimd)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

use core::arch::global_asm;

use interrupt::{switch_to_el_1, init_exception_vector};
use peripherals::{uart_init, Framebuffer, uart_send, uart_send_number};
use process::load_programs;

mod interrupt;
mod peripherals;
mod process;
mod utils;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    //switch_to_el_1();
    uart_init();
    init_exception_vector();

    //let fb = Framebuffer::new();

    //for i in 0..100 {
        //fb.draw_pixel(20, i, 0xFF00FF);
    //}

    load_programs();


    loop {
    }
}


