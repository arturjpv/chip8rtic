use chip8vm;

pub struct Keypad {}

impl Keypad {
    pub fn new() -> Keypad {
        Keypad {}
    }

    pub fn check(&self) {}
}

impl chip8vm::Keypad for Keypad {
    fn is_pressed(&self, _keycode: u8) -> bool {
        false
    }

    fn pressed_key(&self) -> Option<u8> {
        None
    }
}
