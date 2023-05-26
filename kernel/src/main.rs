#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(stdsimd)]
#![feature(lang_items)]
#![feature(panic_info_message)]
#![no_main]
#![no_std]

use core::arch::global_asm;

use interrupt::{switch_to_el_1, init_exception_vector};
use peripherals::{uart_init, uart_send};
use process::load_programs;

mod interrupt;
mod peripherals;
mod process;
mod utils;

global_asm!(include_str!("boot.s"));

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let message = "I'm a kernel!\n";
    uart_init();
    uart_send(message);

    switch_to_el_1();
    init_exception_vector();
    load_programs();

    loop {
    }
}
