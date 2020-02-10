use std::time::Instant;
use std::time::*;
use std::thread::sleep;

pub struct Clock {
    last_tick: Instant,
    delay: u128,
}

impl Clock {
    pub fn new(frequency: u64) -> Self {
        Self {
            last_tick: Instant::now(),
            delay: (1000 / frequency) as u128,
        }
    }

    pub fn tick(&mut self, sync: bool) -> bool {
        let diff = self.last_tick.elapsed();

        if diff.as_millis() >= self.delay {
            self.last_tick += Duration::from_millis(self.delay as u64);
            true
        } else {
            if sync {
                sleep(Duration::from_millis(self.delay as u64) - diff);
                self.last_tick += Duration::from_millis(self.delay as u64);
                true
            } else {
                false
            }
        }
    }
}