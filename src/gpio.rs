use core::ptr::{read_volatile, write_volatile};

use crate::utils::wait_for_n_cycles;

use self::addresses::{GPFSEL, GPPUD_ENABLE, GPPUDCLK_ENABLE};

pub const GPIO_MAX_PIN:         u32 = 53;

#[cfg(feature = "board_rpi3")]
pub mod addresses {
    pub const MMIO_BASE:            u32 = 0x3F000000;
    pub const GPFSEL0:              u32 = MMIO_BASE + 0x00200000;
    pub const GPFSEL:               u32 = MMIO_BASE + 0x00200004;
    pub const GPPUD_ENABLE:         u32 = MMIO_BASE + 0x00200094;
    pub const GPPUDCLK_ENABLE:      u32 = MMIO_BASE + 0x00200098;
    pub const GPPUPPDN0:            u32 = MMIO_BASE + 0x002000E4;
}

#[cfg(feature = "board_rpi4")]
pub mod addresses {
    pub const MMIO_BASE:            u32 = 0xFE000000;
    pub const GPFSEL0:              u32 = MMIO_BASE + 0x00200000;
    pub const GPFSEL:               u32 = MMIO_BASE + 0x00200004;
    pub const GPPUD_ENABLE:         u32 = MMIO_BASE + 0x00200094;
    pub const GPPUDCLK_ENABLE:      u32 = MMIO_BASE + 0x00200098;
    pub const GPPUPPDN0:            u32 = MMIO_BASE + 0x002000E4;
    
}

pub fn set_gpio_func(pin_number: u32, gpio_func: u32) {
    // each GPIO function select is for 10 pins and is 4 bytes long
    let function_select_reg = GPFSEL + (pin_number / 10) * 4;

    let mut selector = unsafe {
        read_volatile(function_select_reg as *const u32)
    };

    let first_bit = (pin_number * 3) / 30;
    // clear the three bits indicating the function
    selector &= !(0b111 << first_bit);
    // set the respective function
    selector |= gpio_func << first_bit;

    unsafe {
        write_volatile(function_select_reg as *mut u32, selector);
    }
}

pub fn gpio_call(pin: u32, value: u32, dst: u32, field_size: u32) {
    let field_mask: u32 = (11 << field_size) -1;
    let num_fields: u32 = 32 / field_size;
    let reg = dst + ((pin / num_fields) * 4);
    let shift = (pin % num_fields) * field_size;
    
    let mut current_value = unsafe {
        read_volatile(reg as *const u32)
    };

    current_value &= !(field_mask << shift);
    current_value |= value << shift;

    unsafe {
        write_volatile(reg as *mut u32, current_value);
    }
}

pub fn enable_gpio_pin(pin_number: u32) {
    disable_gpio_pupd();

    // enable pull up/down for clock
    let selector = 1 << pin_number;
    change_gpio_clock_pupd_register(pin_number, selector);
    disable_gpio_pupd();
    change_gpio_clock_pupd_register(pin_number, 0);
}

fn disable_gpio_pupd() {
    // Disable GPIO pull down/ pull up
    unsafe {
        write_volatile(GPPUD_ENABLE as *mut u32, 0b00);
    }
    wait_for_n_cycles(150);
}

fn change_gpio_clock_pupd_register(pin_number: u32, value: u32) {
    let gpio_clock_pupd_enable_register = GPPUDCLK_ENABLE + pin_number * 4 / 32;
    unsafe {
        write_volatile(gpio_clock_pupd_enable_register as *mut u32, value);
    }
    wait_for_n_cycles(150);
}

pub fn enable_blink() {
    unsafe { 
        write_volatile(0xFE20_0008 as *mut u32, 1<<3);
    }
}

pub fn blink_on() {
    unsafe {
        write_volatile(0xFE20_001C as *mut u32, 1<<21);
    }
}

pub fn blink_off() {
    unsafe {
        write_volatile(0xFE20_0028 as *mut u32, 1<<21);
    }
}

