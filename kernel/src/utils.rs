use core::arch::asm;

pub fn _wait_for_n_cycles(num_cycles: u32) {
    unsafe {
        for _ in 0..num_cycles {
                asm!("wfe");
        }
    }
}

