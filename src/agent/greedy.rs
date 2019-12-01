use num_traits::ToPrimitive;

use crate::Stepper;

use super::{Agent, ArgBounds};

pub struct GreedyAgent<'a> {
    q_star: Vec<f64>,
    stepper: &'a mut dyn Stepper,
}

impl<'a, T: ToPrimitive> Agent<T> for GreedyAgent<'a> {
    fn action(&self) -> usize {
        self.q_star.arg_max()
    }

    fn q_star(&self) -> &Vec<f64> {
        &self.q_star
    }

    fn reset(&mut self, q_init: Vec<f64>) {
        self.q_star = q_init;
        self.stepper.reset()
    }

    fn step(&mut self, arm: usize, reward: T) -> () {
        self.q_star[arm] += self.update(arm, reward)
    }

    fn stepper(&mut self) -> &mut dyn Stepper {
        self.stepper
    }
}

impl<'a> GreedyAgent<'a> {
    pub fn new(q_init: Vec<f64>, stepper: &mut dyn Stepper) -> GreedyAgent {
        GreedyAgent {
            q_star: q_init,
            stepper,
        }
    }
}
