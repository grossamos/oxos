#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write_volatile};
use core::arch::{global_asm, asm};

use oxos_syscall::{uart_send, exit, DisplayBuffer};

global_asm!(include_str!("init.s"));

#[no_mangle]
pub extern "C" fn main() {
    uart_send("Hello Woooorld\n");

    let display = DisplayBuffer::new();
    display.draw_str("INITIALIZING OXOS", 0xFFFFFF, 290, 350);

    for _ in 0..500000 {
        unsafe {asm!("nop")};
    }
    exit();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        write_volatile(0xFE20_001C as *mut u32, 1<<21);
    }
    loop {}
}
