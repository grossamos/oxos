use core::ptr::{write_volatile, read_volatile};

const MMIO_BASE:            u32 = 0x3F000000;

const AUX_BASE:             u32 = MMIO_BASE + 0x00215000;
const AUX_MU_IO_REG:        u32 = AUX_BASE + 0x40;
const AUX_MU_LSR_REG:       u32 = AUX_BASE + 0x54;


pub fn uart_send(message: &str) {
    for letter in message.chars() {
        uart_send_letter(letter as u32);
    }
}

fn uart_send_letter(letter: u32) {
    unsafe {
        // wait while FIFO is full
        while read_volatile(AUX_MU_LSR_REG as *const u32) & 0x20 != 0x20 {}
        write_volatile(AUX_MU_IO_REG as *mut u32, letter as u32);
    }

}

//pub fn uart_send(message: &str) {
    //for letter in message.chars() {
        //uart_send_letter(letter);
    //}
//}

//fn uart_send_letter(letter: char) {
    //unsafe {
        //// wait while FIFO is full
        //while read_volatile(AUX_MU_LSR_REG as *const u32) & 0x20 != 0x20 {}
        //write_volatile(AUX_MU_IO_REG as *mut u32, letter as u32);
    //}

//}

