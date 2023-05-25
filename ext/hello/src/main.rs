#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write_volatile};
use core::arch::{global_asm, asm};

use uart::uart_send;

mod uart;

global_asm!(include_str!("init.s"));

#[no_mangle]
pub extern "C" fn main() -> ! {
    uart_send("I'm an instance!\n");

    unsafe {
        asm!("svc 0x80");
    }

    uart_send("I'm back bitch");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        write_volatile(0xFE20_001C as *mut u32, 1<<21);
    }
    loop {}
}
