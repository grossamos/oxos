use core::{ptr::{read_volatile, write_volatile}, arch::asm};

#[cfg(feature = "board_rpi3")]
pub mod addresses {
    pub const MMIO_BASE:            u32 = 0x3F000000;
}

#[cfg(feature = "board_rpi4")]
pub mod addresses {
    pub const MMIO_BASE:            u32 = 0xFE000000;
}

pub use self::addresses::MMIO_BASE;

use super::{uart_send_number, uart_send};

pub const GPFSEL0:              u32 = MMIO_BASE + 0x00200000;
pub const _GPFSEL1:             u32 = MMIO_BASE + 0x00200004;
pub const _GPFSEL2:             u32 = MMIO_BASE + 0x00200008;
pub const GPSET0:               u32 = MMIO_BASE + 0x0020001C;
pub const _GPCLR0:              u32 = MMIO_BASE + 0x00200028;
pub const GPPUD_ENABLE:         u32 = MMIO_BASE + 0x00200094;
pub const GPPUDCLK_ENABLE:      u32 = MMIO_BASE + 0x00200098;
pub const _GPIO_MAX_PIN:        u32 = 53;
pub const GPLEV1:               u32 = MMIO_BASE + 0x00200034;

pub fn set_gpio_func(pin_number: u32, gpio_func: u32) {
    // each GPIO function select is for 10 pins and is 4 bytes long
    let function_select_reg = GPFSEL0 + (pin_number / 10) * 4;

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
    for _ in 0..150 {
        unsafe {
            asm!("nop");
        }
    }
    //wait_for_n_cycles(150);
}

fn change_gpio_clock_pupd_register(pin_number: u32, value: u32) {
    let gpio_clock_pupd_enable_register = GPPUDCLK_ENABLE + (pin_number / 32) * 4;
    unsafe {
        write_volatile(gpio_clock_pupd_enable_register as *mut u32, value);
    }
    for _ in 0..150 {
        unsafe {
            asm!("nop");
        }
    }
    //wait_for_n_cycles(150);
}

pub fn _enable_blink() {
    unsafe { 
        write_volatile(_GPFSEL2 as *mut u32, 1<<3);
    }
}

pub fn _blink_on() {
    unsafe {
        write_volatile(GPSET0 as *mut u32, 1<<21);
    }
}

pub fn _blink_off() {
    unsafe {
        write_volatile(_GPCLR0 as *mut u32, 1<<21);
    }
}

fn get_gpio_value(pin_number: u32) -> bool {
    let value = unsafe {
        read_volatile(GPLEV1 as *const u32)
    };
    (value & 1 << pin_number) > 0
}

pub fn activate_input_for_gpio(reg: u32) {
    const INPUT_FUNCTION: u32 = 0;
    set_gpio_func(reg, INPUT_FUNCTION);
}

pub fn wait_for_gpio_flip(pin_number: u32) {
    static mut LAST_VAL: bool = false;

    let mut val = get_gpio_value(pin_number);
    // wait until flip
    while unsafe {
        val == LAST_VAL
    } {
        val = get_gpio_value(pin_number);
    }

    unsafe {
        LAST_VAL = val;
    };
}

pub fn get_selected_gpio_regs(regs: [bool; 32]) -> [Option<bool>; 32] {
    let mut retrieved_values = [None; 32];

    for i in 0..regs.len() {
        if regs[i] {
            retrieved_values[i] = Some(get_gpio_value(i as u32));
            if retrieved_values[i] == Some(true) {
                uart_send("true: ");
                uart_send_number(i as u64);
            } else {
                uart_send("false: ");
                uart_send_number(i as u64);
            }
        }
    }

    retrieved_values
}
