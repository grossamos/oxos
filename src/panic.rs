use core::panic::PanicInfo;

use crate::uart::uart_send;

#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    uart_send("KERNEL PANIC!");
    loop {}
} 

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
