#![feature(asm_const)]
#![feature(stdsimd)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::arch::global_asm;
use uart::{uart_init, uart_send};

mod uart;
mod utils;
mod framebuffer;

#[no_mangle]
pub extern fn kernel_main() {
    let message = "Hello World! It'sa me Amosio!\n";

    uart_init();
    uart_send(message);

    loop {
    }
}

global_asm!(include_str!("boot.s"));
  
#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
   loop {}
} 

