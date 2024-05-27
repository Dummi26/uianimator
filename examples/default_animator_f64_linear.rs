use std::time::{Duration, Instant};

use uianimator::{Animator, DefaultAnimatorF64Linear};

fn main() {
    let mut animator = DefaultAnimatorF64Linear::new(0.0, 2.0);
    eprintln!("Initial value: {}", animator.get_value(Instant::now()));
    std::thread::sleep(Duration::from_millis(200));
    let now = Instant::now();
    animator.set_target(1.0, now);
    // unchanged no time has elapsed since the `set_target`, since we use the same `now` for both calls
    eprintln!(
        "Immediately after `set_target`: {}",
        animator.get_value(now)
    );
    eprintln!(
        "Shortly after `set_target`: {}",
        animator.get_value(Instant::now())
    );
    std::thread::sleep(Duration::from_millis(200));
    eprintln!("After ~200ms: {}", animator.get_value(Instant::now()));
    std::thread::sleep(Duration::from_millis(200));
    eprintln!("After ~400ms: {}", animator.get_value(Instant::now()));
    std::thread::sleep(Duration::from_millis(200));
    eprintln!(
        "After ~600ms: {} (target reached)",
        animator.get_value(Instant::now())
    );
}
