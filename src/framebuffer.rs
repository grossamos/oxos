use core::ptr::{read_volatile, write_volatile};

use crate::{gpio::MMIO_BASE, utils::wait_for_n_cycles, uart::uart_send};

const VIDEOCORE_MBOX: u32 = MMIO_BASE + 0x0000B880;
const MBOX_READ: u32 = VIDEOCORE_MBOX + 0x0;
const MBOX_WRITE: u32 = VIDEOCORE_MBOX + 0x20;
const MBOX_STATUS: u32 = VIDEOCORE_MBOX + 0x18;

const MBOX_FULL_FLAG: u32 = 0x80000000;
const MBOX_EMPTY_FLAG: u32 = 0x40000000;

#[repr(C, align(16))]
struct Mbox {
    buffer: [u32; 36],
}

impl Mbox {
    fn new() -> Mbox {
        return Mbox { 
            buffer: [0; 36],
            //buffer: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] 
        }
    }

    fn send(&self, channel: u8) {
        let mbox_channel_addr = ((self.buffer.as_ptr() as u32) & !0xF) | (channel as u32 & 0xF);
        unsafe {
            while read_volatile(MBOX_STATUS as *const u32) & MBOX_FULL_FLAG != 0 {
                wait_for_n_cycles(1);
            }

            write_volatile(MBOX_WRITE as *mut u32, mbox_channel_addr);

            loop {
                while read_volatile(MBOX_STATUS as *const u32) & MBOX_EMPTY_FLAG != 0 {
                    wait_for_n_cycles(1);
                }
                if read_volatile(MBOX_READ as *const u32) == mbox_channel_addr {
                    return;
                }
            }
        }
    }
}

pub struct Framebuffer {
    width: u32,
    height: u32,
    pitch: u32,
    is_rgb: bool,
    address: u32,
}

impl Framebuffer {
   pub fn new() -> Framebuffer {
        let mut mbox = Mbox::new();

        mbox.buffer[0] = 36 * 4;   // buffer len
        mbox.buffer[1] = 0;        // request

        // set physical width + height
        mbox.buffer[2]  = 0x00048003;
        mbox.buffer[3]  = 8;
        mbox.buffer[4]  = 8;
        mbox.buffer[5]  = 1024;     // set width
        mbox.buffer[6]  = 768;      // set height


        // set virtual width + height
        mbox.buffer[7]  = 0x48004;
        mbox.buffer[8]  = 8;
        mbox.buffer[9]  = 8;
        mbox.buffer[10] = 1024;
        mbox.buffer[11] = 768;

        // set depth (bits per pixel)
        mbox.buffer[12] = 0x00048005;
        mbox.buffer[13] = 4;
        mbox.buffer[14] = 4;
        mbox.buffer[15] = 32;

        // set pixel order
        mbox.buffer[16] = 0x00040006;
        mbox.buffer[17] = 4;
        mbox.buffer[18] = 4;
        mbox.buffer[19] = 1;

        // set virtual offset
        mbox.buffer[20] = 0x00048009;
        mbox.buffer[21] = 8;
        mbox.buffer[22] = 8;
        mbox.buffer[23] = 0; // no x offset
        mbox.buffer[24] = 0; // no y offset

        // allocate framebuffer
        mbox.buffer[25] = 0x00040001;
        mbox.buffer[26] = 8;
        mbox.buffer[27] = 8;
        mbox.buffer[28] = 4096;
        mbox.buffer[29] = 0;

        // get pitch
        mbox.buffer[30] = 0x40008;
        mbox.buffer[31] = 4;
        mbox.buffer[32] = 4;
        mbox.buffer[33] = 0;

        mbox.buffer[34] = 0;
        mbox.buffer[35] = 0;

        mbox.send(8);
        
        // temporarily save framebuffer 
        let fb = Framebuffer {
            width: mbox.buffer[5],
            height: mbox.buffer[6],
            pitch: mbox.buffer[33],
            is_rgb: mbox.buffer[19] == 1,
            address: mbox.buffer[28] & 0x3FFFFFFF,
        };

        uart_send("Read Framebuffer");

        fb
    }

   pub fn draw_pixel(&self, x: u32, y: u32) {
       unsafe {
           let offset = y * self.pitch + x * 4;
           write_volatile((self.address + offset) as *mut u32, 0xFFFFFF);
           //write_volatile(self.address as *mut u32, 0x0000FF);
      }
   }

   pub fn draw_hello(&self) {
       for i in 50..200 {
            self.draw_pixel(50, i);
            self.draw_pixel(70, i);
       }

       for i in 45..75 {
           self.draw_pixel(i, 100);
       }

       for i in 50..70 {
           self.draw_pixel(90, i);
       }

       for i in 80..200 {
           self.draw_pixel(90, i);
       }

       for i in 50..150 {
           self.draw_pixel(110, i);
       }

       for i in 170..200 {
           self.draw_pixel(110, i);
       }
   }
}

