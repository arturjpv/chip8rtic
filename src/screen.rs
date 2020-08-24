use stm32f3xx_hal::gpio::{gpiob::PB6, gpiob::PB7, AF4};
use stm32f3xx_hal::hal::blocking::i2c::Write;
use stm32f3xx_hal::i2c::I2c;
use stm32f3xx_hal::pac::I2C1;

use chip8vm;
use chip8vm::{SCREEN_HEIGHT, SCREEN_WIDTH};

const PAGE_SIZE: usize = 8;
const DISPLAY_WIDTH: usize = SCREEN_WIDTH * 2;
const DISPLAY_HEIGHT: usize = SCREEN_HEIGHT * 2;
const DISPLAY_SIZE: usize = (DISPLAY_WIDTH * DISPLAY_HEIGHT) / PAGE_SIZE;

pub struct Screen {
    pixels: [u8; DISPLAY_SIZE],
    stencil: [u8; DISPLAY_SIZE],
    dirty: [u8; PAGE_SIZE],
    i2c: I2c<I2C1, (PB6<AF4>, PB7<AF4>)>,
}

impl Screen {
    pub fn new(i2c: I2c<I2C1, (PB6<AF4>, PB7<AF4>)>) -> Screen {
        Screen {
            pixels: [0u8; DISPLAY_SIZE],
            stencil: [1u8; DISPLAY_SIZE],
            dirty: [1u8; PAGE_SIZE],
            i2c,
        }
    }

    pub fn init(&mut self) {
        self.i2c.write(0x3C, &[0x00, 0xAE]).unwrap(); // Display Off
        self.i2c.write(0x3C, &[0x00, 0xD5, 0x80]).unwrap(); // Display Clock Div
        self.i2c.write(0x3C, &[0x00, 0xA8, 0x3E]).unwrap(); // Multiplex
        self.i2c.write(0x3C, &[0x00, 0xD3, 0x00]).unwrap(); // Display Offset
        self.i2c.write(0x3C, &[0x00, 0x40, 0x00]).unwrap(); // Start Line
        self.i2c.write(0x3C, &[0x00, 0xAD, 0x8B]).unwrap(); // Charge Pump
        self.i2c.write(0x3C, &[0x00, 0xDA, 0x12]).unwrap(); // Common Pads Pin Config
        self.i2c.write(0x3C, &[0x00, 0xC8]).unwrap(); // Segment Remap
        self.i2c.write(0x3C, &[0x00, 0xA1]).unwrap(); // Reverse COM Dir
        self.i2c.write(0x3C, &[0x00, 0x81, 0x80]).unwrap(); // Contrast
        self.i2c.write(0x3C, &[0x00, 0xD9, 0xF1]).unwrap(); // Pre Charge Period
        self.i2c.write(0x3C, &[0x00, 0xDB, 0x35]).unwrap(); // Common Voltage Level Deselect
        self.i2c.write(0x3C, &[0x00, 0xA4]).unwrap(); // All on
        self.i2c.write(0x3C, &[0x00, 0xA6]).unwrap(); // Invert
        self.i2c.write(0x3C, &[0x00, 0xAF]).unwrap(); // Display On
    }

    pub fn display(&mut self) {
        let mut paint_cmd: [u8; 2] = [0; 2];
        paint_cmd[0] = 0x40;

        for page in 0..PAGE_SIZE {
            if self.dirty[page] != 0 {
                self.i2c.write(0x3C, &[0x00, 0xB0 | page as u8]).unwrap(); // Page Address

                for column in 0..DISPLAY_WIDTH {
                    let position = page * DISPLAY_WIDTH + column;
                    if self.stencil[position] != 0 {
                        self.stencil[position] = 0;
                        paint_cmd[1] = self.pixels[position];

                        self.i2c
                            .write(0x3C, &[0x00, 0x0F & (column + 2) as u8])
                            .unwrap(); // Column Address Low
                        self.i2c
                            .write(0x3C, &[0x00, 0x10 | ((column + 2) as u8 >> 4)])
                            .unwrap(); // Column Address High
                        self.i2c.write(0x3C, &paint_cmd).unwrap(); // Display Data
                    }
                }
            }

            self.dirty[page] = 0;
        }
    }

    fn put_pixel(&mut self, x: u8, y: u8) -> bool {
        let page = y as usize / PAGE_SIZE;
        let byte: usize = page * DISPLAY_WIDTH + x as usize;
        let bit = y as usize % PAGE_SIZE;
        let mask = 0b00000001 << bit;

        let collision = self.pixels[byte] & mask != 0;
        self.pixels[byte] ^= mask;
        self.stencil[byte] |= mask;
        self.dirty[page] = 1;

        collision
    }
}

impl chip8vm::Screen for Screen {
    fn clear(&mut self) {
        for i in 0..DISPLAY_SIZE {
            self.pixels[i] = 0;
            self.stencil[i] = 1;
        }

        for i in 0..PAGE_SIZE {
            self.dirty[i] = 1;
        }
    }

    fn draw(&mut self, x: u8, y: u8) -> bool {
        self.put_pixel(x * 2, y * 2);
        self.put_pixel(x * 2, y * 2 + 1);
        self.put_pixel(x * 2 + 1, y * 2);
        self.put_pixel(x * 2 + 1, y * 2 + 1)
    }
}
