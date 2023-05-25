use core::arch::{asm, global_asm};

use crate::{uart::{uart_send, uart_send_number}, batch_loader::{clean_stack_from_last_program, jump_to_next_program}};

// same number used in linux
const SYSCALL_EXIT_NUM: u64         = 0x80;

const SCTLR_RESERVED: u64           = (3 << 28) | (3 << 22) | (1 << 20) | (1 << 11);
const SCTLR_EE_LITTLE_ENDIAN: u64   = 0 << 25;
const _SCTLR_EOE_LITTLE_ENDIAN: u64  = 0 << 24;
const SCTLR_I_CACHE_DISABLED: u64   = 0 << 12;
const SCTLR_D_CACHE_DISABLED: u64   = 0 << 2;
const SCTLR_MMU_DISABLED: u64       = 0 << 0;
const _SCTLR_MMU_ENABLED: u64        = 1 << 0;
pub const SCTLR_VALUE_MMU_DISABLED: u64 = SCTLR_RESERVED | SCTLR_EE_LITTLE_ENDIAN | SCTLR_I_CACHE_DISABLED | SCTLR_D_CACHE_DISABLED | SCTLR_MMU_DISABLED;

const SPSR_MASK_ALL: u64            = 7 << 6;
const SPSR_EL1H: u64                = 5 << 0;
pub const SPSR_VALUE: u64               = SPSR_MASK_ALL | SPSR_EL1H;

global_asm!(include_str!("./exception.s"));

// The exception context as it is stored on the stack on exception entry.
#[repr(C)]
struct ExceptionContext {
    // General Purpose Registers.
    gpr: [u64; 30],

    // The link register, aka x30.
    lr: u64,

    // Exception link register. The program counter at the time the exception happened.
    elr_el1: u64,

    // saved program status register
    spsr_el1: u64,

    // exception syndrome register
    esr_el1: u64,

}

#[no_mangle]
extern "C" fn current_elx_synchronous(e: &mut ExceptionContext) {
    uart_send("Welcome to your exception...\n");
    let fp_val: *const u64;
    unsafe {
        asm!("mov {}, x29", out(reg) fp_val);
    }
    unsafe {
        uart_send_number(*fp_val);
    }
    uart_send_number(e.esr_el1);
    if e.esr_el1 & 0xFFFF == SYSCALL_EXIT_NUM {
        syscall_exit(e);
    } else {
        uart_send_number(e.esr_el1 & 0xFFFF);
        uart_send("\nUnknown Kernel Function!\n");
        panic!("Unknown kernel function");
    }
}

fn syscall_exit(_e: &mut ExceptionContext) {
    // Clean stack
    clean_stack_from_last_program();
    
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

pub fn get_exception_level() -> u64 {
    let mut exception_level: u64;
    unsafe {
        asm!("mrs {0:x}, CurrentEL", out(reg) exception_level);
    }
    (exception_level & 0b1100) >> 2
}

pub fn switch_to_el_1() {
    unsafe {
        let sctrl_el1_val = SCTLR_VALUE_MMU_DISABLED;
        asm!("msr sctlr_el1, {}", in(reg) sctrl_el1_val);

        let lsr_el2_val: u64 = 1 << 31;
        asm!("msr hcr_el2, {}", in(reg) lsr_el2_val);

        let spr_val = SPSR_VALUE;
        asm!("msr spsr_el2, {}", in(reg) spr_val);
        
        let mut adr_save: u64;
        asm!( 
            "mov {}, x0",
            "adr x0, 2f",
            "msr elr_el2, x0",
            "eret",
            "2: nop",
            out(reg) adr_save,
        );

        asm!(
            "mov x0, {}", in(reg) adr_save
        );

    }
}

pub fn init_exception_vector() {
    extern "C" {
        static mut __exception_vector: u64;
    } 

    let stack_el1: u64 = 0x80000 - 0x1000;
    unsafe {
        asm!("msr vbar_el1, {}", in(reg) &__exception_vector as *const u64);
        asm!("mov sp, {}", in(reg) stack_el1);
    }
}
