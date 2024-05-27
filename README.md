# uianimator

_**Smooth, interruptable animations for your UI**_

This crate provides the `Animator` trait and some default animators.
You can use the `get_value` and `set_target` functions on an animator to interact with it.

## Why?

UI animations usually suffer from one of two problems:

**They cannot be interrupted** \
If a user opens a sidebar which has a slide-in animation, but they close it before it is fully open,
the sidebar might jump to its fully-open state before playing the slide-out animation,
or it will immediately start moving in the opposite direction, going from `+x` speed to `-x` without any transition.

or **they are inaccurate and influenced by frame-/tickrate** \
The first problem can be solved somewhat easily by storing a `speed` and `position`, then using an iterative algorithm to update them.
However, this means that running the application at a lower framerate will make the animation slower.
You can work around this by using `deltaTime`, or otherwise moving in bigger steps when more time has passed,
but this will still make your animations inconsistent across devices which perform differently.

uianimator solves both of these problems.
it doesn't use an iterative approach and all the default animators are designed so that the both your animations current value (position)
and the rate at which it changes (speed) will not jump, but instead transition smoothly, so that your animations never seem jerky.

## Usage

```rust
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
```

The initial `value` of the animator is `0.5`.
It then start moving towards `2.0`, but never reaches it, because a user interrupts the animation, setting the new `target` to `0`. \
This is the program's output (the right line represents the value `2`, the left one is `0`):


```
val: 0.50 | =========================                                                                            |
val: 0.51 | =========================                                                                            |
val: 0.54 | ===========================                                                                          |
val: 0.59 | =============================                                                                        |
val: 0.66 | =================================                                                                    |
val: 0.75 | =====================================                                                                |
val: 0.87 | ===========================================                                                          |
val: 1.00 | =================================================                                                    |
val: 1.15 | =========================================================                                            |
val: 1.30 | =================================================================                                    |
val: 1.43 | =======================================================================                              |
val: 1.55 | =============================================================================                        |
val: 1.64 | =================================================================================                    |
val: 1.71 | =====================================================================================                |
val: 1.76 | =======================================================================================              |
val: 1.79 | =========================================================================================            |
val: 1.80 | ==========================================================================================           |
val: 1.79 | =========================================================================================            |
val: 1.76 | =======================================================================================              |
val: 1.71 | =====================================================================================                |
val: 1.64 | =================================================================================                    |
val: 1.55 | =============================================================================                        |
val: 1.43 | =======================================================================                              |
val: 1.30 | =================================================================                                    |
val: 1.15 | =========================================================                                            |
val: 0.98 | ================================================                                                     |
val: 0.79 | =======================================                                                              |
val: 0.62 | ===============================                                                                      |
val: 0.47 | =======================                                                                              |
val: 0.35 | =================                                                                                    |
val: 0.24 | ===========                                                                                          |
val: 0.15 | =======                                                                                              |
val: 0.08 | ====                                                                                                 |
val: 0.03 | =                                                                                                    |
val: 0.01 |                                                                                                      |
val: 0.00 |                                                                                                      |
```
