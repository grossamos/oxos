#![no_std]
#![no_main]
#![feature(core_intrinsics)]
#![feature(stdsimd)]

use core::panic::PanicInfo;

mod uart;
mod utils;

#[no_mangle]
pub extern fn kernel_main() {
    uart::uart_init();

    loop {
    }
}
  
#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
   loop {}
} 

