use std::time::Instant;

use speedy2d::color::Color;
use speedy2d::dimen::{UVec2, Vec2};
use speedy2d::shape::Rectangle;
use speedy2d::window::{MouseButton, WindowHandler, WindowHelper};
use speedy2d::{Graphics2D, Window};
use uianimator::default_animator_f64_quadratic::DefaultAnimatorF64Quadratic;
use uianimator::Animator;

fn main() {
    let window = Window::new_centered("uianimator example", (1280, 720)).unwrap();
    window.run_loop(MyWindowHandler {
        anim: DefaultAnimatorF64Quadratic::new(100.0, 30.0),
        now: Instant::now,
        conv: |v| v as _,
        rconv: |v| v as _,
        size: UVec2::ZERO,
        mouse_pos: Vec2::ZERO,
        mouse_down: false,
    });
}

struct MyWindowHandler<
    A: Animator,
    F: Fn() -> A::Time,
    G: Fn(A::Value) -> f32,
    H: Fn(f32) -> A::Value,
> {
    anim: A,
    now: F,
    conv: G,
    rconv: H,
    size: UVec2,
    mouse_pos: Vec2,
    mouse_down: bool,
}

impl<A: Animator, F: Fn() -> A::Time, G: Fn(A::Value) -> f32, H: Fn(f32) -> A::Value> WindowHandler
    for MyWindowHandler<A, F, G, H>
{
    fn on_draw(&mut self, helper: &mut WindowHelper, graphics: &mut Graphics2D) {
        graphics.clear_screen(Color::BLACK);
        graphics.draw_rectangle(
            Rectangle::from_tuples(
                (0.0, self.size.y as f32 * 0.25),
                (
                    (self.conv)(self.anim.get_value((self.now)())),
                    self.size.y as f32 * 0.75,
                ),
            ),
            Color::WHITE,
        );
        helper.request_redraw();
    }
    fn on_resize(&mut self, helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.size = size_pixels;
        helper.request_redraw();
    }
    fn on_mouse_move(&mut self, helper: &mut WindowHelper<()>, position: Vec2) {
        self.mouse_pos = position;
        // if self.mouse_down {
        //     self.anim
        //         .set_target((self.rconv)(self.mouse_pos.x), (self.now)());
        // }
        helper.request_redraw();
    }
    fn on_mouse_button_down(&mut self, helper: &mut WindowHelper<()>, _button: MouseButton) {
        self.anim
            .set_target((self.rconv)(self.mouse_pos.x), (self.now)());
        self.mouse_down = true;
        helper.request_redraw();
    }
    fn on_mouse_button_up(&mut self, helper: &mut WindowHelper<()>, _button: MouseButton) {
        self.mouse_down = false;
        helper.request_redraw();
    }
}
