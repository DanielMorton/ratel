use num_traits::ToPrimitive;
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::uniform::Uniform;

use crate::Stepper;

use super::{Agent, ArgBounds};

struct EpsilonGreedyAgent<'a> {
    q_star: Vec<f64>,
    stepper: &'a mut dyn Stepper,
    epsilon: f64,
    uniform: Uniform<f64>,
    pick_arm: Uniform<usize>,
}

impl<'a, T: ToPrimitive> Agent<T> for EpsilonGreedyAgent<'a> {
    fn action(&self) -> usize {
        if self.uniform.sample(&mut thread_rng()) < self.epsilon {
            self.pick_arm.sample(&mut thread_rng())
        } else {
            self.q_star.arg_max()
        }
    }

    fn q_star(&self) -> &Vec<f64> {
        &self.q_star
    }

    fn reset(&mut self, q_init: Vec<f64>) {
        self.q_star = q_init;
        self.stepper.reset()
    }

    fn step(&mut self, arm: usize, reward: T) -> () {
        self.q_star[arm] += self.stepper.step(arm) * (reward.to_f64().unwrap() - self.q_star[arm])
    }
}

impl<'a> EpsilonGreedyAgent<'a> {
    fn new(q_init: Vec<f64>, stepper: &mut dyn Stepper, epsilon: f64) -> EpsilonGreedyAgent {
        assert!(epsilon > 0.0);
        assert!(epsilon < 1.0);
        let l = q_init.len();
        EpsilonGreedyAgent {
            q_star: q_init,
            stepper,
            epsilon,
            uniform: Uniform::new(0.0, 1.0),
            pick_arm: Uniform::new(0usize, l),
        }
    }
}
