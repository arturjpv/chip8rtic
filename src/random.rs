use chip8vm;

pub struct Random {
    state: u64,
    increment: u64,
}

// *Really* minimal PCG32 code / (c) 2014 M.E. O'Neill / pcg-random.org
// Licensed under Apache License 2.0 (NO WARRANTY, etc. see website)
impl Random {
    pub fn new() -> Random {
        Random {
            state: 42,
            increment: 54,
        }
    }

    pub fn rand(&mut self) -> u32 {
        let old_state = self.state;
        // Advance internal state
        self.state = old_state
            .overflowing_mul(6364136223846793005)
            .0
            .overflowing_add(self.increment | 1)
            .0;
        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted: u32 = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot: i64 = (old_state >> 59) as i64;
        ((xorshifted >> rot as u32) | (xorshifted << ((-rot) & 31) as u32)) as u32
    }
}

impl chip8vm::Random for Random {
    fn range(&mut self) -> u8 {
        (self.rand() % u8::max_value() as u32) as u8
    }
}
