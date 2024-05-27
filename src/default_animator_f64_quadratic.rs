use std::time::Instant;

use crate::Animator;

pub struct DefaultAnimatorF64Quadratic {
    /// this value squared * 2 = difference between starting point (@ time - elapsed_duration) and target
    target_reach_half_duration: f64,
    elapsed_duration: f64,
    decreasing: bool,
    start: f64,
    time: Instant,
    target: f64,
    /// in units per second per second
    speed: f64,
}
impl DefaultAnimatorF64Quadratic {
    pub fn new(value: f64, speed: f64) -> Self {
        Self {
            target_reach_half_duration: 0.0,
            elapsed_duration: 0.0,
            decreasing: false,
            start: value,
            time: Instant::now(),
            target: value,
            speed,
        }
    }
}
impl Animator for DefaultAnimatorF64Quadratic {
    type Value = f64;
    type Time = Instant;

    fn get_value(&self, time: Self::Time) -> Self::Value {
        let elapsed = self.elapsed_duration
            + self.speed * time.saturating_duration_since(self.time).as_secs_f64();
        if elapsed < self.target_reach_half_duration {
            // speed up
            if self.decreasing {
                self.start - elapsed * elapsed
            } else {
                self.start + elapsed * elapsed
            }
        } else {
            // slow down
            let remaining = (self.target_reach_half_duration * 2.0) - elapsed;
            if remaining > 0.0 {
                if self.decreasing {
                    self.target + remaining * remaining
                } else {
                    self.target - remaining * remaining
                }
            } else {
                self.target
            }
        }
    }
    fn set_target(&mut self, target: Self::Value, time: Self::Time) {
        if self.target == target {
            return;
        }
        let elapsed = self.elapsed_duration
            + self.speed * time.saturating_duration_since(self.time).as_secs_f64();
        let mut set_time = true;
        // dbg!(elapsed, self.target_reach_half_duration);
        self.start = if elapsed < self.target_reach_half_duration {
            let slow_down_now_end_value = if self.decreasing {
                self.start - elapsed * elapsed * 2.0
            } else {
                self.start + elapsed * elapsed * 2.0
            };
            let new_decreasing = if self.decreasing {
                target <= slow_down_now_end_value
            } else {
                target < slow_down_now_end_value
            };
            // speeding up
            if new_decreasing == self.decreasing {
                // we should speed up
                set_time = false;
                self.start
            } else {
                // should slow down, then go reverse
                self.decreasing = new_decreasing;
                // so that the new curve has the same slope, but changes in the opposite direction
                self.elapsed_duration = -elapsed;
                slow_down_now_end_value
            }
        } else {
            let remaining = (self.target_reach_half_duration * 2.0) - elapsed;
            if remaining > 0.0 {
                // slowing down
                let new_decreasing = if self.decreasing {
                    target <= self.target
                } else {
                    target < self.target
                };
                if new_decreasing != self.decreasing {
                    // should slow down, then go reverse
                    self.decreasing = new_decreasing;
                    // so that the new curve has the same slope, but changes in the opposite direction
                    self.elapsed_duration = -remaining;
                    self.target
                } else {
                    // should speed up, then slow down
                    self.decreasing = new_decreasing;
                    // so that the new curve has the same slope, but changes in the opposite direction
                    self.elapsed_duration = remaining;
                    if self.decreasing {
                        self.target + remaining * remaining * 2.0
                    } else {
                        self.target - remaining * remaining * 2.0
                    }
                }
            } else {
                // target reached
                self.elapsed_duration = 0.0;
                self.decreasing = target < self.target;
                self.target
            }
        };
        self.target = target;
        self.target_reach_half_duration = (0.5 * (self.start - self.target).abs()).sqrt();
        if set_time {
            self.time = time;
        }
    }
}
