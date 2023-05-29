use core::arch::asm;

use crate::uart_send;

pub struct DisplayBuffer {
}

impl DisplayBuffer {
    pub fn new() -> DisplayBuffer {
        DisplayBuffer {  }
    }
    pub fn draw_pixel(&self, x: u32, y: u32, color: u32) { 
        //uart_send("test");
        unsafe {
            asm!(
                "sub sp, sp, 16 * 5",
                "stp x1, x2, [sp, 16 * 0]",
                "stp x3, x8, [sp, 16 * 1]",
                "mov w1, {0:w}",
                "mov w2, {1:w}",
                "mov w3, {2:w}",
                "mov x8, 0x82",
                "svc 0x00",
                in(reg) x,
                in(reg) y,
                in(reg) color,
            );
            asm!(
                "ldp x1, x2, [sp, 16 * 0]",
                "ldp x3, x8, [sp, 16 * 1]",
                "add sp, sp, 16 * 5",
            );
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
