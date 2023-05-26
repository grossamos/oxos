#![no_std]
#![no_main]

use core::{panic::PanicInfo, ptr::write_volatile};
use core::arch::{global_asm, asm};

global_asm!(include_str!("init.s"));

#[no_mangle]
pub extern "C" fn main() {
    let message = "I'm an instance\n";

    unsafe {
        let mut addr_save_0: u64;
        let mut addr_save_1: u64;
        let mut addr_save_2: u64;
        asm!(
            "mov {}, x1",
            "mov {}, x2",
            "mov {}, x8",
            "mov x1, {}",
            "mov x2, {}",
            "mov x8, 0x81",
            "svc 0x00",
            out(reg) addr_save_0,
            out(reg) addr_save_1,
            out(reg) addr_save_2,
            in(reg) message.len(),
            in(reg) message.as_ptr() as u64,
        );
        asm!(
            "mov x1, {}",
            "mov x2, {}",
            "mov x8, {}",
            in(reg) addr_save_0,
            in(reg) addr_save_1,
            in(reg) addr_save_2,
        );
    }

    unsafe {
        asm!(
            "mov x8, 0x80",
            "svc 0x00"
        );
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        write_volatile(0xFE20_001C as *mut u32, 1<<21);
    }
    loop {}
}
