use oxos_syscall::DisplayBuffer;

pub struct PlayingField {
    display: DisplayBuffer,
}

pub enum Symbol {
    Cross,
    Circle
}

enum Orientation {
    Horizontal,
    Vertical
}

impl PlayingField {
    pub fn new() -> PlayingField {
        let display = DisplayBuffer::new();
        return PlayingField { display }
    }

    pub fn draw_playing_field(&self) {
        self.display.clear_screen();
        self.draw_title();

        self.draw_line(205, 205 * 1 + 100, 41 * 3, 5, Orientation::Horizontal, 0xFFFFFF);
        self.draw_line(205, 205 * 2 + 100, 41 * 3, 5, Orientation::Horizontal, 0xFFFFFF);

        self.draw_line(205 * 2, 100, 41 * 3, 5, Orientation::Vertical, 0xFFFFFF);
        self.draw_line(205 * 3, 100, 41 * 3, 5, Orientation::Vertical, 0xFFFFFF);

    }

    pub fn draw_field_entry(&self, x: u32, y: u32, symbol: Symbol, color: u32) {
        let x = x * 205 + 205 + 102;
        let y = y * 205 + 202;

        match symbol {
            Symbol::Cross => self.draw_x(x, y, color),
            Symbol::Circle => self.draw_circle(x, y, color),
        }
    }

    fn draw_title(&self) {
        self.display.draw_str("TIC TAC TOE OF DOOM", 0xFF00FF, 270, 10);
    }


    fn draw_line(&self, x: u32, y: u32, len: u32, block_size: u32, orientation: Orientation, color: u32) {
        for i in 0..len {
            match orientation {
                Orientation::Horizontal => self.draw_block(x + i * block_size, y, block_size, color),
                Orientation::Vertical => self.draw_block(x, y + i * block_size, block_size, color)
            }
        }
    }

    const DIAMETER: i32 = 70;

    fn draw_x(&self, x: u32, y: u32, color: u32) {
        for i in 0..Self::DIAMETER {
            self.draw_block((x as i32 + i) as u32, (y as i32 + i) as u32, 4, color);
            self.draw_block((x as i32 - i) as u32, (y as i32 - i) as u32, 4, color);
            self.draw_block((x as i32 + i) as u32, (y as i32 - i) as u32, 4, color);
            self.draw_block((x as i32 - i) as u32, (y as i32 + i) as u32, 4, color);
        }

    }

    fn draw_circle(&self, x: u32, y: u32, color: u32) {

        // Bresenham circle
        let mut x_offset: i32 = 0;
        let mut y_offset: i32 = Self::DIAMETER;

        let mut descision = 3 - 2 * Self::DIAMETER;
        loop {
            self.draw_block((x as i32 + x_offset) as u32, (y as i32 + y_offset) as u32, 4, color);
            self.draw_block((x as i32 + x_offset) as u32, (y as i32 - y_offset) as u32, 4, color);
            self.draw_block((x as i32 - x_offset) as u32, (y as i32 + y_offset) as u32, 4, color);
            self.draw_block((x as i32 - x_offset) as u32, (y as i32 - y_offset) as u32, 4, color);

            self.draw_block((x as i32 + y_offset) as u32, (y as i32 + x_offset) as u32, 4, color);
            self.draw_block((x as i32 + y_offset) as u32, (y as i32 - x_offset) as u32, 4, color);
            self.draw_block((x as i32 - y_offset) as u32, (y as i32 + x_offset) as u32, 4, color);
            self.draw_block((x as i32 - y_offset) as u32, (y as i32 - x_offset) as u32, 4, color);

            if x_offset >= y_offset {
                break;
            }

            if descision < 0 {
                x_offset = x_offset + 1;
                y_offset = y_offset;
                descision = descision + 4 * x_offset + 6;
            } else {
                x_offset = x_offset + 1;
                y_offset = y_offset - 1;
                descision = descision + 4 * (x_offset - y_offset) + 10;
            }
        }
    }

    fn draw_block(&self, x: u32, y: u32, block_size: u32, color: u32) {
        for i in 0..block_size {
            for j in 0..block_size {
                self.display.draw_pixel(x + i, y + j, color)
            }
        }
    }

    pub fn valid_play(&self) {
    }

    pub fn inindicate_invalid_play(&self) {
    }
}

