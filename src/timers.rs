use std::thread;
use std::time::Duration;

pub struct ChipTimer {
    delay_timer: u8,
    sound_timer: u8,
}

// This function should be run once at some point in main.rs
pub fn start_timer_thread() {
    // Timers decrement at a steady 60Hz
    let timer_interval = Duration::from_secs(1) / 60;

    let mut timerStruct = ChipTimer {
        delay_timer: 0,
        sound_timer: 0,
    };

    let t = thread::spawn(move || {
        loop {
            if timerStruct.delay_timer > 0 {
                timerStruct.delay_timer -= 1;
            }

            if timerStruct.sound_timer > 0 {
                timerStruct.sound_timer -= 1;
            }

            println!("Timers ticked");

            thread::sleep(timer_interval);
        }
    });
}
