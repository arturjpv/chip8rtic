use chip8vm;

pub struct Screen {}

impl Screen {
    pub fn new() -> Screen {
        Screen {}
    }

    pub fn display(&self) {}
}

impl chip8vm::Screen for Screen {
    fn clear(&mut self) {}

    fn draw(&mut self, _x: u8, _y: u8) -> bool {
        false
    }
}
