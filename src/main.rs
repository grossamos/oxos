#![feature(asm_const)]
#![feature(format_args_nl)]
#![feature(stdsimd)]
#![feature(lang_items)]
#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::arch::global_asm;
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
    //fb.draw_pixel(10, 10);
    //fb.draw_pixel(20, 20);
    fb.draw_hello();
    uart_send("More text, did it work?");

    loop {
    }
}

global_asm!(include_str!("boot.s"));
  
#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    uart_send("KERNEL PANIC!");
    loop {}
} 

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
