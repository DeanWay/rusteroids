use std::time::Duration;

use bevy::time::Timer;

pub struct Cooldown {
    available: bool,
    timer: Timer,
}

impl Cooldown {
    pub fn new(duration: Duration) -> Self {
        Self {
            available: true,
            timer: Timer::new(duration, false),
        }
    }

    pub fn available(&self) -> bool {
        self.available
    }

    pub fn trigger(&mut self) {
        self.available = false;
    }

    pub fn tick(&mut self, delta: Duration) {
        if !self.available {
            self.timer.tick(delta);
            if self.timer.finished() {
                self.timer.reset();
                self.available = true;
            }
        }
    }
}
