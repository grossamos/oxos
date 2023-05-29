#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write_volatile};
use core::arch::global_asm;

use oxos_syscall::{uart_send, exit, DisplayBuffer, activate_input_gpio, wait_for_gpio_to_flip, get_gpio_values};

global_asm!(include_str!("init.s"));

#[no_mangle]
pub extern "C" fn main() {
    uart_send("Hello Woooorld\n");

    activate_input_gpio(20);
    wait_for_gpio_to_flip(20);
    let mut gpio_pins = [false; 32];
    gpio_pins[20] = true;
    gpio_pins[1] = true;

    let values = get_gpio_values(gpio_pins);
    match values[20] {
        Some(true) => uart_send("\ntrue\n"),
        Some(false) => uart_send("\nfalse\n"),
        None => uart_send("no val"),
    };
    match values[1] {
        Some(true) => uart_send("\ntrue\n"),
        Some(false) => uart_send("\nfalse\n"),
        None => uart_send("no val"),
    };

    uart_send("Got it\n");

    let display = DisplayBuffer::new();
    display.draw_str("HELLO WORLD!");

    uart_send("\nDONE WITH PROGRAM!");
    exit();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        write_volatile(0xFE20_001C as *mut u32, 1<<21);
    }
    loop {}
}
