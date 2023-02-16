use core::ptr::{write_volatile, read_volatile};

use crate::gpio::{MMIO_BASE, set_gpio_func, enable_gpio_pin};

const AUX_ENABLES:          u32 = MMIO_BASE + 0x00215004;
const AUX_MU_CNTL_REG:      u32 = MMIO_BASE + 0x00215060;
const AUX_MU_IER_REG:       u32 = MMIO_BASE + 0x00215044;
const AUX_MU_LCR_REG:       u32 = MMIO_BASE + 0x0021504C;
const AUX_MU_MCR_REG:       u32 = MMIO_BASE + 0x00215050;
const AUX_MU_BAUD_REG:      u32 = MMIO_BASE + 0x00215068;
const AUX_MU_LSR_REG:       u32 = MMIO_BASE + 0x00215054;
const AUX_MU_IO_REG:        u32 = MMIO_BASE + 0x00215040;

const TXD_GPIO_PIN:         u32 = 14;
const RXD_GPIO_PIN:         u32 = 15;
const ALT_FUC_UART:         u32 = 5;


pub fn uart_init() {
    set_gpio_func(TXD_GPIO_PIN, ALT_FUC_UART);
    set_gpio_func(RXD_GPIO_PIN, ALT_FUC_UART);
    enable_gpio_pin(TXD_GPIO_PIN);
    enable_gpio_pin(RXD_GPIO_PIN);

    // Initialize mini UART
    unsafe {
        write_volatile(AUX_ENABLES as *mut u32, 1);
        write_volatile(AUX_MU_CNTL_REG as *mut u32, 0); // disable everything (while configuring)
        write_volatile(AUX_MU_IER_REG as *mut u32, 0); // disable interrupts for uart
        write_volatile(AUX_MU_LCR_REG as *mut u32, 0b11); // 8 bit mode
        write_volatile(AUX_MU_MCR_REG as *mut u32, 0); // RTS to allways high
        write_volatile(AUX_MU_BAUD_REG as *mut u32, 270); // rate dependent on system clock frequency
        write_volatile(AUX_MU_CNTL_REG as *mut u32, 0b11); // enable read and transmit
    }

    uart_send("\r\n\n");
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
