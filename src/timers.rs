use std::thread;
use std::time::Duration;

// Arc: Allows thread-safe shared ownership
// MutEx: Mutal exclusive access
use std::sync::{Arc, Mutex};

pub struct ChipTimer {
    delay_timer: u8,
    sound_timer: u8,
}

impl ChipTimer {
    pub fn new() -> Self {

        return ChipTimer {
            delay_timer: 0,
            sound_timer: 0,
        };
    }

    // Delay timer
    pub fn read_dt(&self) -> u8 {
        return self.delay_timer;
    }
    pub fn write_dt(&mut self, val: u8) {
        self.delay_timer = val;
    }

    // Sound timer
    pub fn read_st(&self) -> u8 {
        return self.sound_timer;
    }
    pub fn write_st(&mut self, val: u8) {
        self.sound_timer = val;
    }
}
