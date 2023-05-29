use core::{arch::asm, ptr::write_volatile, slice::from_raw_parts, str::from_utf8};

use crate::{process::{jump_to_next_program, INSTANCE_STACK_BASE, switch_to_instance_stack}, peripherals::{uart_send, Framebuffer, uart_send_number, activate_input_for_gpio, wait_for_gpio_flip, get_selected_gpio_regs}};

use super::exception_handler::ExceptionContext;

// same number used in linux
const SYSCALL_EXIT: u64                     = 0x80;
const SYSCALL_UART: u64                     = 0x81;
const SYSCALL_DRAW_PIXEL: u64               = 0x82;

const SYSCALL_ACTIVATE_INPUT_REGISTER: u64  = 0x83;
const SYSCALL_WAIT_FOR_GPIO_FLIP: u64       = 0x84;
const SYSCALL_GET_REG_VALUE: u64            = 0x85;

// The exception context as it is stored on the stack on exception entry.
#[no_mangle]
extern "C" fn execute_syscall(e: &mut ExceptionContext) {
    match e.gpr[8] {
        SYSCALL_EXIT => syscall_exit(e),
        SYSCALL_UART => syscall_uart_send(e),
        SYSCALL_DRAW_PIXEL => syscall_draw_pixel(e),
        SYSCALL_ACTIVATE_INPUT_REGISTER => syscall_activate_input_register(e),
        SYSCALL_WAIT_FOR_GPIO_FLIP => syscall_wait_for_gpio_flip(e),
        SYSCALL_GET_REG_VALUE => syscall_get_reg_value(e),
        _ => {
            for i in 0..30 {
                uart_send_number(e.gpr[i]);
            }
            uart_send("\nSyscall number:");
            uart_send_number(e.gpr[8]);
            uart_send("ESR_EL1:");
            uart_send_number(e.esr_el1);
            uart_send("ELR_EL1:");
            uart_send_number(e.elr_el1);
            panic!("Unknown kernel function");
        },
    }
}


fn syscall_exit(e: &mut ExceptionContext) {
    // Clean stack of process
    for i in e.sp..INSTANCE_STACK_BASE {
        unsafe {
            write_volatile(i as *mut u32, 0);
        }
    }

    switch_to_instance_stack();
    
    // Return from exception
    unsafe {
        asm!( 
            "adr x0, 2f",
            "msr elr_el2, x0",
            "eret",
            "2: nop",
        );
    }

    // jump to next program
    jump_to_next_program();
}

fn syscall_uart_send(e: &mut ExceptionContext) {
    let length = e.gpr[1];
    unsafe {
        let message: &[u8] = from_raw_parts(e.gpr[2] as *const u8, length as usize);
        let message: &str = match from_utf8(message) {
            Ok(message) => message,
            Err(_) => panic!("Uart print failed due to utf8 issues"),
        };

        uart_send(message);
    }
    //uart_send("Sending...\n");
}

static mut FRAMEBUFFER: Option<Framebuffer> = None;

fn syscall_draw_pixel(e: &mut ExceptionContext) {
    // initialize fb 
    unsafe {
        match FRAMEBUFFER {
            Some(_) => (),
            None => {
                FRAMEBUFFER = Some(Framebuffer::new());
            }
        }

        match &FRAMEBUFFER {
            Some(fb) => {
                fb.draw_pixel(e.gpr[1] as u32, e.gpr[2] as u32, e.gpr[3] as u32)
            },
            None => panic!("Failed to initialize Framebuffer"),
        }
    }
}

fn syscall_activate_input_register(e: &mut ExceptionContext) {
    activate_input_for_gpio(e.gpr[1] as u32);
}

fn syscall_wait_for_gpio_flip(e: &mut ExceptionContext) {
    wait_for_gpio_flip(e.gpr[1] as u32);
}

fn syscall_get_reg_value(e: &mut ExceptionContext) {
    let mut regs = [false; 32];

    uart_send_number(e.gpr[1]);
    for i in 0..32 {
        if e.gpr[1] & (1 << i) > 0 {
            regs[i] = true;
        }
    }

    let values = get_selected_gpio_regs(regs);

    let mut result = 0;
    
    for i in 0..32 {
        match values[i] {
            Some(true) => result = result | (1 << i),
            _ => {},
        }
    }

    unsafe {
        asm!("mov x0, {0:x}", in(reg) result)
    }
}
