use std::time::{Duration, Instant};

use uianimator::{default_animator_f64_quadratic::DefaultAnimatorF64Quadratic, Animator};

fn main() {
    // start at 0.5 with a speed factor of 2.
    let mut animator = DefaultAnimatorF64Quadratic::new(0.5, 2.0);
    // smoothly transition from 0.5 to 2
    animator.set_target(2.0, Instant::now());
    loop {
        // repeatedly get the animator's current value and print it
        let val = animator.get_value(Instant::now());
        let count = (50.0 * val) as _;
        eprintln!(
            "val: {val:.2} | {}{} |",
            "=".repeat(count),
            " ".repeat(100 - count)
        );
        // once we reach 1, go to 0 instead. this simulates a user interrupting our animation.
        if val > 1.0 {
            animator.set_target(0.0, Instant::now());
        }
        // once we reach 0, exit
        if val == 0.0 {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
