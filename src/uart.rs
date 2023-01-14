use core::ptr::{write_volatile, read_volatile};

use crate::{utils::wait_for_n_cycles, gpio::{MMIO_BASE, GPFSEL1, GPPUD, GPPUDCLK0}};

const AUX_ENABLES:          u32 = MMIO_BASE + 0x00215004;
const AUX_MU_CNTL_REG:      u32 = MMIO_BASE + 0x00215060;
const AUX_MU_IER_REG:       u32 = MMIO_BASE + 0x00215044;
const AUX_MU_LCR_REG:       u32 = MMIO_BASE + 0x0021504C;
const AUX_MU_MCR_REG:       u32 = MMIO_BASE + 0x00215050;
const AUX_MU_BAUD_REG:      u32 = MMIO_BASE + 0x00215068;
const AUX_MU_LSR_REG:       u32 = MMIO_BASE + 0x00215054;
const AUX_MU_IO_REG:        u32 = MMIO_BASE + 0x00215040;

pub fn uart_init() {
    // for more information: see chapter 6 in https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf

    // Set GPIO function to UART
    let mut selector = unsafe {
        read_volatile(GPFSEL1 as *const u32)
    };

    selector &= !(0b111 << 12); // clear bits 12-14 for gpio 14
    selector |= 0b010 << 12; // select alt5 for gpio 14
    selector &= !(0b111 << 15); // clear bits 15-17 for gpio 15
    selector |= 0b010 << 15; // select alt5 for gpio 15

    unsafe {
        write_volatile(GPFSEL1 as *mut u32, selector);
    }

    // Disable GPIO pull down/ pull up
    unsafe {
        write_volatile(GPPUD as *mut u32, 0b00);
    }
    wait_for_n_cycles(150);

    let selector = (1 << 14) | (1 << 15); // select pins 14 and 15

    unsafe {
        write_volatile(GPPUDCLK0 as *mut u32, selector);
    }
    wait_for_n_cycles(150);

    unsafe {
        write_volatile(GPPUDCLK0 as *mut u32, 0);
    }

    // Initialize mini UART
    unsafe {
        write_volatile(AUX_ENABLES as *mut u32, 1);
        write_volatile(AUX_MU_CNTL_REG as *mut u32, 0); // disable everything (while configuring)
        write_volatile(AUX_MU_IER_REG as *mut u32, 0); // disable interrupts for uart
        write_volatile(AUX_MU_LCR_REG as *mut u32, 0b11); // 8 bit mode
        write_volatile(AUX_MU_MCR_REG as *mut u32, 0); // RTS to allways high
        write_volatile(AUX_MU_BAUD_REG as *mut u32, 270); // rate dependent on system clock frequency
        write_volatile(AUX_MU_CNTL_REG as *mut u32, 0b11); // enable read and transmut
    }
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
