use f3::hal::gpio::gpiob::{PB6, PB7};
use f3::hal::gpio::AF4;
use f3::hal::i2c::I2c;
use f3::hal::stm32f30x::I2C1;

use sh1106::prelude::*;
use sh1106::Builder;

use chip8vm;
use chip8vm::{SCREEN_HEIGHT, SCREEN_WIDTH};

const SCREEN_SIZE: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) / 8;

pub struct Screen {
    pixels: [u8; SCREEN_SIZE],
    stencil: [u8; SCREEN_SIZE],
    display: GraphicsMode<I2cInterface<I2c<I2C1, (PB6<AF4>, PB7<AF4>)>>>,
}

impl Screen {
    pub fn new(i2c: I2c<I2C1, (PB6<AF4>, PB7<AF4>)>) -> Screen {
        let mut display: GraphicsMode<_> = Builder::new().connect_i2c(i2c).into();

        display.init().unwrap();
        display.flush().unwrap();

        Screen {
            pixels: [0u8; SCREEN_SIZE],
            stencil: [0u8; SCREEN_SIZE],
            display,
        }
    }

    pub fn display(&mut self) {
        for y in 0..SCREEN_HEIGHT as u32 {
            for x in 0..SCREEN_WIDTH as u32 {
                let position = y as usize * SCREEN_WIDTH + x as usize;
                let byte = position / 8;
                let bit = position % 8;

                if self.stencil[byte] & (0b10000000 >> bit) != 0 {
                    self.stencil[byte] ^= 0b10000000 >> bit;

                    let color: u8 = self.pixels[byte] & (0b10000000 >> bit);
                    self.display.set_pixel(x * 2, y * 2, color);
                    self.display.set_pixel(x * 2, y * 2 + 1, color);
                    self.display.set_pixel(x * 2 + 1, y * 2, color);
                    self.display.set_pixel(x * 2 + 1, y * 2 + 1, color);
                }
            }
        }

        self.display.flush().unwrap();
    }
}

impl chip8vm::Screen for Screen {
    fn clear(&mut self) {}

    fn draw(&mut self, x: u8, y: u8) -> bool {
        let position = y as usize * SCREEN_WIDTH + x as usize;
        let byte = position / 8;
        let bit = position % 8;

        let collision = self.pixels[byte] & (0b10000000 >> bit) != 0;
        self.pixels[byte] ^= 0b10000000 >> bit;
        self.stencil[byte] |= 0b10000000 >> bit;

        collision
    }
}
