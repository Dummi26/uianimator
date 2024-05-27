pub mod default_animator_f64_linear;
pub mod default_animator_f64_quadratic;

pub trait Animator {
    type Value;
    type Time;

    fn get_value(&self, time: Self::Time) -> Self::Value;
    fn set_target(&mut self, target: Self::Value, time: Self::Time);
}
