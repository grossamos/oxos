#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write_volatile};
use core::arch::global_asm;

use oxos_syscall::{uart_send, exit, DisplayBuffer};

global_asm!(include_str!("init.s"));

#[no_mangle]
pub extern "C" fn main() {
    let message = "Hello World!\n";
    uart_send(message);
    let display = DisplayBuffer::new();
    display.draw_pixel(0, 0, 0xFFFF00);
    //display.draw_str("hello world");

    exit();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        write_volatile(0xFE20_001C as *mut u32, 1<<21);
    }
    loop {}
}
