use core::ptr::{write_volatile, read_volatile};

use crate::gpio::{set_gpio_func, enable_gpio_pin, addresses::{MMIO_BASE, GPPUPPDN0, GPFSEL0}, gpio_call};

const AUX_BASE:             u32 = MMIO_BASE + 0x00215000;
const AUX_ENABLES:          u32 = AUX_BASE + 0x4;
const AUX_MU_IO_REG:        u32 = AUX_BASE + 0x40;
const AUX_MU_IER_REG:       u32 = AUX_BASE + 0x44;
const AUX_MU_IIR_REG:       u32 = AUX_BASE + 0x48;
const AUX_MU_LCR_REG:       u32 = AUX_BASE + 0x4C;
const AUX_MU_MCR_REG:       u32 = AUX_BASE + 0x50;
const AUX_MU_LSR_REG:       u32 = AUX_BASE + 0x54;
const AUX_MU_CNTL_REG:      u32 = AUX_BASE + 0x60;
const AUX_MU_BAUD_REG:      u32 = AUX_BASE + 0x68;

const TXD_GPIO_PIN:         u32 = 14;
const RXD_GPIO_PIN:         u32 = 15;
const ALT_FUC_UART:         u32 = 5;

#[cfg(feature = "board_rpi3")]
pub mod baud {
    pub const BAUD_RATE:    u32 = 270;
}

#[cfg(feature = "board_rpi4")]
pub mod baud {
    pub const BAUD_RATE:    u32 = 541;
}


pub fn uart_init() {
    //set_gpio_func(TXD_GPIO_PIN, ALT_FUC_UART);
    //set_gpio_func(RXD_GPIO_PIN, ALT_FUC_UART);
    //enable_gpio_pin(TXD_GPIO_PIN);
    //enable_gpio_pin(RXD_GPIO_PIN);

    // Initialize mini UART
    unsafe {
        write_volatile(AUX_ENABLES as *mut u32, 1);
        write_volatile(AUX_MU_IER_REG as *mut u32, 0); // disable interrupts for uart
        write_volatile(AUX_MU_CNTL_REG as *mut u32, 0); // disable everything (while configuring)
        write_volatile(AUX_MU_LCR_REG as *mut u32, 0b11); // 8 bit mode
        write_volatile(AUX_MU_MCR_REG as *mut u32, 0); // RTS to allways high
        write_volatile(AUX_MU_IIR_REG as *mut u32, 0xC6); // RTS to allways high
        write_volatile(AUX_MU_BAUD_REG as *mut u32, baud::BAUD_RATE); // rate dependent on system clock frequency
        init_uart_pin(14);
        init_uart_pin(15);
        write_volatile(AUX_MU_CNTL_REG as *mut u32, 0b11); // enable read and transmit
    }

    uart_send("\r\n\n");
}

pub fn init_uart_pin(pin: u32) {
    gpio_call(pin, 0, GPPUPPDN0, 2);
    gpio_call(pin, 2, GPFSEL0, 2);
}

pub fn uart_send(message: &str) {
    for letter in message.chars() {
        uart_send_letter(letter);
    }
}

fn uart_send_letter(letter: char) {
    unsafe {
        // wait while FIFO is full
        while read_volatile(AUX_MU_LSR_REG as *const u32) & 0x20 != 0x20 {}
        write_volatile(AUX_MU_IO_REG as *mut u32, letter as u32);
    }

}
