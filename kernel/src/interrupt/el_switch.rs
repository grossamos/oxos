use core::arch::asm;

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

pub fn _get_execution_level() -> u64 {
    let mut execution_level: u64;
    unsafe {
        asm!("mrs {0:x}, CurrentEL", out(reg) execution_level);
    }
    (execution_level & 0b1100) >> 2
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
