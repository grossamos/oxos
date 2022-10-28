use core::intrinsics::{volatile_load, volatile_store};

use crate::utils::wait_for_n_cycles;

// would be 0xFE000000 for raspberry pi 4
const MMIO_BASE:    u32 = 0x3F000000;
const GPFSEL1:      u32 = 0x3F200004;
const GPPUD:        u32 = 0x3F200094;
const GPPUDCLK0:    u32 = 0x3F200098;
const AUX_ENABLES:  u32 = 0x3F215004;

pub fn uart_init() {
    // for more information: see chapter 6 in https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf

    // Set GPIO function to UART
    let mut selector = unsafe {
        volatile_load(GPFSEL1 as *const u32)
    };

    selector &= !(0b111 << 12); // clear bits 12-14 for gpio 14
    selector |= 0b010 << 12; // select alt5 for gpio 14
    selector &= !(0b111 << 15); // clear bits 15-17 for gpio 15
    selector |= 0b010 << 15; // select alt5 for gpio 15

    unsafe {
        volatile_store(GPFSEL1 as *mut u32, selector);
    }

    // Disable GPIO pull down/ pull up
    unsafe {
        volatile_store(GPPUD as *mut u32, 0x00);
    }
    wait_for_n_cycles(150);

    let mut selector = 0;
    selector |= 11 << 14; // select pins 14 and 15

    unsafe {
        volatile_store(GPPUDCLK0 as *mut u32, selector);
    }
    wait_for_n_cycles(150);

    unsafe {
        volatile_store(GPPUDCLK0 as *mut u32, 0);
    }

    // Initialize mini UART
    unsafe {
        volatile_store(AUX_ENABLES as *mut u32, 1);
    }

}
