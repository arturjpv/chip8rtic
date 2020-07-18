use f3::led::Leds;

pub struct Blinker {
    pub led: usize,
}

impl Blinker {
    pub fn run(&mut self, led: &mut Leds) {
        led[self.led].off();
        self.led = (self.led + 1) % 8;
        led[self.led].on();
    }
}
