#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern fn kernel_main() {
    let mut variable = 1;
    loop {
        variable += 1;
    }
}
  
#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
   loop {}
} 

