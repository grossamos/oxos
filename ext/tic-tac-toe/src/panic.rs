use core::panic::PanicInfo;

use oxos_syscall::uart_send;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    uart_send("Process Panic!");
    loop {}
}
