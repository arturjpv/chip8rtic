use chip8vm;

use crate::keypad::KeyState::{PRESSED, RELEASED};
use stm32f3xx_hal::gpio::{gpiod::PD8, gpiod::PD9, Input, PullUp};
use stm32f3xx_hal::hal::digital::v2::InputPin;

pub struct Buttons {
    pub button1: PD8<Input<PullUp>>,
    pub button2: PD9<Input<PullUp>>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    PRESSED,
    RELEASED,
}

pub struct Keypad {
    keys: [KeyState; chip8vm::KEYPAD_NUM_KEYS],
}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {
            keys: [KeyState::RELEASED; chip8vm::KEYPAD_NUM_KEYS],
        }
    }

    pub fn check(&mut self, button1: &impl InputPin, button2: &impl InputPin) {
        match button1.is_low() {
            Ok(true) => self.keys[4] = PRESSED,
            Ok(false) => self.keys[4] = RELEASED,
            Err(_) => {}
        }

        match button2.is_low() {
            Ok(true) => self.keys[6] = PRESSED,
            Ok(false) => self.keys[6] = RELEASED,
            Err(_) => {}
        }
    }
}

impl chip8vm::Keypad for Keypad {
    fn is_pressed(&self, keycode: u8) -> bool {
        match self.keys[keycode as usize] {
            KeyState::PRESSED => true,
            KeyState::RELEASED => false,
        }
    }

    fn pressed_key(&self) -> Option<u8> {
        for (keycode, state) in self.keys.iter().enumerate() {
            match state {
                KeyState::PRESSED => {
                    return Some(keycode as u8);
                }
                KeyState::RELEASED => continue,
            }
        }

        None
    }
}
