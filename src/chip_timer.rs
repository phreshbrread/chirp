use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};

pub struct ChipTimer {
    delay_timer: AtomicU8,
    sound_timer: AtomicU8,
    should_beep: AtomicBool,
}

// Isolate the constructor
impl ChipTimer {
    pub fn new() -> Self {
        return ChipTimer {
            delay_timer: AtomicU8::new(0),
            sound_timer: AtomicU8::new(0),
            should_beep: AtomicBool::new(false),
        };
    }
}

impl ChipTimer {
    pub fn tick(&self) {
        if self.delay_timer.load(Ordering::Relaxed) > 0 {
            self.delay_timer.fetch_sub(1, Ordering::Relaxed);
        }

        if self.sound_timer.load(Ordering::Relaxed) > 0 {
            self.sound_timer.fetch_sub(1, Ordering::Relaxed);
            self.should_beep.store(true, Ordering::Relaxed);
        } else {
            self.should_beep.store(false, Ordering::Relaxed);
        }
    }

    // Delay timer
    pub fn read_dt(&self) -> u8 {
        return self.delay_timer.load(Ordering::Relaxed);
    }
    pub fn write_dt(&self, val: u8) {
        self.delay_timer.store(val, Ordering::Relaxed);
    }

    // Sound timer
    pub fn write_st(&self, val: u8) {
        self.sound_timer.store(val, Ordering::Relaxed);
    }

    pub fn should_beep(&self) -> bool {
        return self.should_beep.load(Ordering::Relaxed);
    }
}
