use core::panic::PanicInfo;

use crate::peripherals::uart_send;

#[panic_handler]
#[no_mangle]
fn panic(panic: &PanicInfo<'_>) -> ! {
    uart_send("KERNEL PANIC!\n");
    match panic.message() {
        Some(message) => {
            match message.as_str() {
                Some(message) => {
                    uart_send(message);
                },
                None => {}
            }
        },
        None => {}
    }

    loop {}
} 

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
