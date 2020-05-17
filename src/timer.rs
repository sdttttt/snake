

use std::thread;
use std::time::Duration;
use crate::game_object::refresh;

pub struct Timer {
    clock: u64
}

impl Timer {
    pub fn new(clock: u64) -> Self {
        Self {
            clock
        }
    }

    pub fn execute(&self) {
        loop {
            thread::sleep(Duration::from_millis(self.clock));
            refresh();
        }
    }
}