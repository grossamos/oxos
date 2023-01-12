#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(stdsimd)]
#![no_main]
#![no_std]

extern crate compiler_builtins;

use core::panic::PanicInfo;

use framebuffer::Framebuffer;
use uart::{uart_init, uart_send};

mod uart;
mod utils;
mod framebuffer;
mod gpio;

#[no_mangle]
pub extern fn kernel_main() {
    let message = "Hello World! It'sa me Amosio!\n";

    uart_init();
    uart_send(message);

    let fb = Framebuffer::new();
    fb.draw_pixel(0, 0);

    loop {
    }
}

  
#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
   loop {}
} 

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
