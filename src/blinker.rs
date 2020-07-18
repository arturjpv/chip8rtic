use f3::led::Leds;

pub struct Blinker {
    pub led: usize,
    pub compass: Leds,
}

impl Blinker {
    pub fn new(compass: Leds) -> Blinker {
        Blinker { led: 7, compass }
    }

    pub fn run(&mut self) {
        self.compass[self.led].off();
        self.led = (self.led + 1) % 8;
        self.compass[self.led].on();
    }
}
