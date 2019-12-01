use num_traits::ToPrimitive;

use crate::Stepper;

pub trait Agent<T: ToPrimitive> {
    fn action(&self) -> usize;

    fn arms(&self) -> usize {
        self.q_star().len()
    }

    fn current_estimate(&self, arm: usize) -> f64 {
        self.q_star()[arm]
    }

    fn q_star(&self) -> &Vec<f64>;

    fn reset(&mut self, q_init: Vec<f64>) -> ();

    fn step(&mut self, arm: usize, reward: T) -> ();

    fn update(&mut self, arm: usize, reward: T) -> f64 {
        self.stepper().step(arm) * (reward.to_f64().unwrap() - self.q_star()[arm])
    }

    fn stepper(&mut self) -> &mut dyn Stepper;
}
