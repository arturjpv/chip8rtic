use chip8vm;

use crate::keypad::KeyState::{PRESSED, RELEASED};
use stm32f3xx_hal::gpio::{
    gpiod::PD10, gpiod::PD11, gpiod::PD12, gpiod::PD13, gpiod::PD14, gpiod::PD15, gpiod::PD8,
    gpiod::PD9, Input, PullUp,
};
use stm32f3xx_hal::hal::digital::v2::InputPin;

pub struct Buttons {
    pub button1: PD8<Input<PullUp>>,
    pub button2: PD9<Input<PullUp>>,
    pub button3: PD10<Input<PullUp>>,
    pub button4: PD11<Input<PullUp>>,
    pub button5: PD12<Input<PullUp>>,
    pub button6: PD13<Input<PullUp>>,
    pub button7: PD14<Input<PullUp>>,
    pub button8: PD15<Input<PullUp>>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum KeyState {
    PRESSED,
    RELEASED,
}

pub struct Keypad {
    keys: [KeyState; chip8vm::KEYPAD_NUM_KEYS],
    keymap: [u8; 8],
}

impl Keypad {
    pub fn new(keymap: [u8; 8]) -> Keypad {
        Keypad {
            keys: [KeyState::RELEASED; chip8vm::KEYPAD_NUM_KEYS],
            keymap,
        }
    }

    pub fn check(
        &mut self,
        button1: &impl InputPin,
        button2: &impl InputPin,
        button3: &impl InputPin,
        button4: &impl InputPin,
        button5: &impl InputPin,
        button6: &impl InputPin,
        button7: &impl InputPin,
        button8: &impl InputPin,
    ) {
        // Left Pad 1
        match button1.is_low() {
            Ok(true) => self.keys[self.keymap[0] as usize] = PRESSED,
            Ok(false) => self.keys[self.keymap[0] as usize] = RELEASED,
            Err(_) => {}
        }

        match button2.is_low() {
            // Right Pad 1
            Ok(true) => self.keys[self.keymap[1] as usize] = PRESSED,
            Ok(false) => self.keys[self.keymap[1] as usize] = RELEASED,
            Err(_) => {}
        }

        // Up Pad 1
        match button3.is_low() {
            Ok(true) => self.keys[self.keymap[2] as usize] = PRESSED,
            Ok(false) => self.keys[self.keymap[2] as usize] = RELEASED,
            Err(_) => {}
        }

        // Down Pad 1
        match button4.is_low() {
            Ok(true) => self.keys[self.keymap[3] as usize] = PRESSED,
            Ok(false) => self.keys[self.keymap[3] as usize] = RELEASED,
            Err(_) => {}
        }

        // Left Pad 2
        match button5.is_low() {
            Ok(true) => self.keys[self.keymap[4] as usize] = PRESSED,
            Ok(false) => self.keys[self.keymap[4] as usize] = RELEASED,
            Err(_) => {}
        }

        // Right Pad 2
        match button6.is_low() {
            Ok(true) => self.keys[self.keymap[5] as usize] = PRESSED,
            Ok(false) => self.keys[self.keymap[5] as usize] = RELEASED,
            Err(_) => {}
        }

        // Up Pad 2
        match button7.is_low() {
            Ok(true) => self.keys[self.keymap[6] as usize] = PRESSED,
            Ok(false) => self.keys[self.keymap[6] as usize] = RELEASED,
            Err(_) => {}
        }

        // Down Pad 2
        match button8.is_low() {
            Ok(true) => self.keys[self.keymap[7] as usize] = PRESSED,
            Ok(false) => self.keys[self.keymap[7] as usize] = RELEASED,
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
