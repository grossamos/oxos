use core::ptr::{read_volatile, write_volatile};

use crate::{gpio::addresses::MMIO_BASE, utils::wait_for_n_cycles};

const VIDEOCORE_MBOX: u32 = MMIO_BASE + 0x0000B880;
const MBOX_READ: u32 = VIDEOCORE_MBOX + 0x0;
const MBOX_WRITE: u32 = VIDEOCORE_MBOX + 0x20;
const MBOX_STATUS: u32 = VIDEOCORE_MBOX + 0x18;

const MBOX_FULL_FLAG: u32 = 0x80000000;
const MBOX_EMPTY_FLAG: u32 = 0x40000000;

#[repr(C, align(32))]
struct Mbox {
    buffer: [u32; 36],
}

impl Mbox {
    fn new() -> Mbox {
        return Mbox { 
            buffer: [0; 36],
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


#[repr(align(32))]
pub struct Framebuffer {
    _width: u32,
    _height: u32,
    pitch: u32,
    _is_rgb: bool,
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
            _width: mbox.buffer[5],
            _height: mbox.buffer[6],
            pitch: mbox.buffer[33],
            _is_rgb: mbox.buffer[19] == 1,
            // TOD change this to fEfffff.. for RPI 4
            address: mbox.buffer[28] & 0x3FFFFFFF,
        };

        // important: length cannot be changed
        //uart_send("Got Framebuffer\n");

        fb
    }

   pub fn draw_pixel(&self, x: u32, y: u32, color: u32) {
       unsafe {
           let offset = y * self.pitch + x * 4;
           write_volatile((self.address + offset) as *mut u32, color);
           //write_volatile(self.address as *mut u32, 0x0000FF);
      }
   }


   pub fn draw_bitmap(&self, bitmap: [[bool; BITMAP_SIZE]; BITMAP_SIZE], x: u32, y: u32, color: u32) {
       for y_index in 0..BITMAP_SIZE {
           //self.draw_pixel(x + (0 as u32), y + (y_index as u32), color);
           for x_index in 0..BITMAP_SIZE {
               if bitmap[y_index][x_index] {
                   for x_padding in 0..BIT_SIZE {
                       for y_padding in 0..BIT_SIZE {
                           self.draw_pixel(x + (x_index as u32) * BIT_SIZE + x_padding, y + (y_index as u32) * BIT_SIZE + y_padding, color)
                       }
                   }
               }
           }
       }
   }

   pub fn draw_str(&self, message: &str) {
       let color = 0xFFFFFF;
       for letter_index in 0..message.len() {
           let x = (letter_index * BITMAP_SIZE) as u32 * BIT_SIZE // account for letter size
                        + BIT_SIZE * (letter_index as u32); // add space between letters
        
           match message.as_bytes()[letter_index] {
               32 /* " " */ => self.draw_bitmap(BITMAP[26], x, BIT_SIZE, color),
               33 /* "!" */ => self.draw_bitmap(BITMAP[27], x, BIT_SIZE, color),
               44 /* "," */ => self.draw_bitmap(BITMAP[28], x, BIT_SIZE, color),
               46 /* "." */ => self.draw_bitmap(BITMAP[29], x, BIT_SIZE, color),
               65..=91 => self.draw_bitmap(BITMAP[message.as_bytes()[letter_index] as usize - 65], x, BIT_SIZE, color),
               _ => (),
           };
       }
   }
}

const O: bool = false;
const X: bool = true;

const BITMAP_SIZE: usize = 5;
const BIT_SIZE: u32 = 4;

const BITMAP: [[[bool; BITMAP_SIZE]; BITMAP_SIZE]; 30] = [
    // A
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
    ],
    // B
    [
        [X, X, X, X, O],
        [X, O, O, O, X],
        [X, X, X, X, O],
        [X, O, O, O, X],
        [X, X, X, X, O],
    ],
    // C
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [X, O, O, O, O],
        [X, O, O, O, X],
        [O, X, X, X, O],
    ],
    // D
    [
        [X, X, X, X, O],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, X, X, X, O],
    ],
    // E
    [
        [O, X, X, X, X],
        [X, O, O, O, O],
        [X, X, X, X, O],
        [X, O, O, O, O],
        [O, X, X, X, X],
    ],
    // F
    [
        [O, X, X, X, X],
        [X, O, O, O, O],
        [X, X, X, X, O],
        [X, O, O, O, O],
        [X, O, O, O, O],
    ],
    // G
    [
        [O, X, X, X, X],
        [X, O, O, O, O],
        [X, O, O, X, X],
        [X, O, O, O, X],
        [O, X, X, X, O],
    ],
    // H
    [
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, X, X, X, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
    ],
    // I
    [
        [O, X, X, X, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, X, X, X, O],
    ],
    // J
    [
        [X, X, X, X, X],
        [O, O, O, X, O],
        [O, O, O, X, O],
        [O, X, O, X, O],
        [O, O, X, O, O],
    ],
    // K
    [
        [X, O, O, O, X],
        [X, O, O, X, O],
        [X, X, X, O, O],
        [X, O, O, X, O],
        [X, O, O, O, X],
    ],
    // L
    [
        [X, O, O, O, O],
        [X, O, O, O, O],
        [X, O, O, O, O],
        [X, O, O, O, O],
        [X, X, X, X, X],
    ],
    // M
    [
        [X, O, O, O, X],
        [X, X, O, X, X],
        [X, O, X, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
    ],
    // N
    [
        [X, O, O, O, X],
        [X, X, O, O, X],
        [X, O, X, O, X],
        [X, O, O, X, X],
        [X, O, O, O, X],
    ],
    // O
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [O, X, X, X, O],
    ],
    // P
    [
        [O, X, X, X, O],
        [X, O, O, X, X],
        [X, X, X, X, O],
        [X, O, O, O, O],
        [X, O, O, O, O],
    ],
    // Q
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, X, X],
        [O, X, X, X, X],
    ],
    // R
    [
        [O, X, X, X, O],
        [X, O, O, O, X],
        [X, X, X, X, O],
        [X, O, X, O, O],
        [X, O, O, X, O],
    ],
    // S
    [
        [O, X, X, X, X],
        [X, O, O, O, O],
        [O, X, X, X, O],
        [O, O, O, O, X],
        [X, X, X, X, O],
    ],
    // T
    [
        [X, X, X, X, X],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
    ],
    // U
    [
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [O, X, X, X, O],
    ],
    // V
    [
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [O, X, O, X, O],
        [O, O, X, O, O],
    ],
    // W
    [
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, O, O, X],
        [X, O, X, O, X],
        [O, X, O, X, O],
    ],
    // X 
    [
        [X, O, O, O, X],
        [O, X, O, X, O],
        [O, O, X, O, O],
        [O, X, O, X, O],
        [X, O, O, O, X],
    ],
    // Y 
    [
        [X, O, O, O, X],
        [X, O, O, O, X],
        [O, X, O, X, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
    ],
    // Z
    [
        [X, X, X, X, X],
        [O, O, O, X, O],
        [O, O, X, O, O],
        [O, X, O, O, O],
        [X, X, X, X, X],
    ],
    // [SPACE]
    [
        [O, O, O, O, O],
        [O, O, O, O, O],
        [O, O, O, O, O],
        [O, O, O, O, O],
        [O, O, O, O, O],
    ],
    // !
    [
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, X, O, O],
        [O, O, O, O, O],
        [O, O, X, O, O],
    ],
    // ,
    [
        [O, O, O, O, O],
        [O, O, O, O, O],
        [O, O, O, O, O],
        [O, O, X, O, O],
        [O, X, O, O, O],
    ],
    // .
    [
        [O, O, O, O, O],
        [O, O, O, O, O],
        [O, O, O, O, O],
        [O, X, X, O, O],
        [O, X, X, O, O],
    ],
];
