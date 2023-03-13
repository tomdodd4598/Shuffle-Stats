use std::num::Wrapping;
use std::ops::Rem;
use std::time::SystemTime;

pub struct PCG {
    state: Wrapping<usize>,
}

impl PCG {
    pub fn new() -> PCG {
        PCG {
            state: Wrapping(SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis() as usize)
        }
    }

    fn internal(&mut self) -> usize {
        let prev = self.state;
        self.state = Wrapping(0x14057b7ef767814f) + prev * Wrapping(0x5851f42d4c957f2d);
        let shifted = ((prev >> 18) ^ prev) >> 27;
        let rot = prev >> 59;

        let mut x = (shifted >> rot.0) | (shifted << ((-rot).0 & 31));
        x = (x ^ (x >> 30)) * Wrapping(0xbf58476d1ce4e5b9);
        x = (x ^ (x >> 27)) * Wrapping(0x94d049bb133111eb);
        (x ^ (x >> 31)).0
    }

    pub fn rand_int<T: From<usize> + Rem<Output = T>>(&mut self, range: T) -> T {
        T::from(self.internal()) % range
    }

    pub fn rand_bool(&mut self) -> bool {
        self.internal() & 1 == 0
    }
}
