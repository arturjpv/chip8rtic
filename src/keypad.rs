use chip8vm;

use stm32f3xx_hal::gpio::gpiod;

pub struct Keypad {}

impl Keypad {
    pub fn new(_input: gpiod::Parts) -> Keypad {
        Keypad {}
    }

    pub fn check(&mut self) {}
}

impl chip8vm::Keypad for Keypad {
    fn is_pressed(&self, _keycode: u8) -> bool {
        false
    }

    fn pressed_key(&self) -> Option<u8> {
        None
    }
}
