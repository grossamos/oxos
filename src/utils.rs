pub fn wait_for_n_cycles(num_cycles: u32) {
    for _ in 0..num_cycles {
        unsafe {
            core::arch::aarch64::__wfe();
        }
    }
}
