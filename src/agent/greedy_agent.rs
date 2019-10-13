use num_traits::ToPrimitive;

use crate::util::Stepper;

use super::{Agent, ArgBounds};

struct GreedyAgent<'a> {
    q_star: Vec<f64>,
    stepper: &'a mut dyn Stepper,
}

impl<'a, T: ToPrimitive> Agent<T> for GreedyAgent<'a> {
    fn action(&self) -> usize {
        self.q_star.arg_max()
    }

    fn arms(&self) -> usize {
        self.q_star.len()
    }

    fn current_estimate(&self, arm: usize) -> f64 {
        self.q_star[arm]
    }

    fn step(&mut self, arm: usize, reward: T) -> () {
        self.q_star[arm] += self.stepper.step(arm) * (reward.to_f64().unwrap() - self.q_star[arm])
    }
}

impl<'a> GreedyAgent<'a> {
    fn new(q_init: Vec<f64>, stepper: &mut dyn Stepper) -> GreedyAgent {
        GreedyAgent { q_star: q_init, stepper }
    }
}