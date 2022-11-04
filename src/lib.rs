#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(stdsimd)]

use core::panic::PanicInfo;

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
  
#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
   loop {}
} 

