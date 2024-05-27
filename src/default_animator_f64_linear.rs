use std::time::Instant;

use crate::Animator;

pub struct DefaultAnimatorF64Linear {
    value: f64,
    time: Instant,
    target: f64,
    /// in units per second
    speed: f64,
}
impl DefaultAnimatorF64Linear {
    pub fn new(value: f64, speed: f64) -> Self {
        Self {
            value,
            time: Instant::now(),
            target: value,
            speed,
        }
    }
}
impl Animator for DefaultAnimatorF64Linear {
    type Value = f64;
    type Time = Instant;

    fn get_value(&self, time: Self::Time) -> Self::Value {
        if self.value < self.target {
            self.target.min(
                self.value + time.saturating_duration_since(self.time).as_secs_f64() * self.speed,
            )
        } else if self.value > self.target {
            self.target.max(
                self.value - time.saturating_duration_since(self.time).as_secs_f64() * self.speed,
            )
        } else {
            self.target
        }
    }
    fn set_target(&mut self, target: Self::Value, time: Self::Time) {
        if self.target == target {
            return;
        }
        self.value = self.get_value(time);
        self.time = time;
        self.target = target;
    }
}
