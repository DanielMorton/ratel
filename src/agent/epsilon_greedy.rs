use std::marker::PhantomData;

use num_traits::ToPrimitive;
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::uniform::Uniform;

use crate::Stepper;

use super::{Agent, ArgBounds};

pub struct EpsilonGreedyAgent<'a, T> {
    q_star: Vec<f64>,
    stepper: &'a mut dyn Stepper,
    epsilon: f64,
    uniform: Uniform<f64>,
    pick_arm: Uniform<usize>,
    phantom: PhantomData<T>,
}

impl<'a, T: ToPrimitive> Agent<T> for EpsilonGreedyAgent<'a, T> {
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
        self.q_star[arm] += self.update(arm, reward)
    }

    fn stepper(&mut self) -> &mut dyn Stepper {
        self.stepper
    }
}

impl<'a, T> EpsilonGreedyAgent<'a, T> {
    pub fn new(q_init: Vec<f64>, stepper: &mut dyn Stepper, epsilon: f64) -> EpsilonGreedyAgent<T> {
        assert!(epsilon > 0.0);
        assert!(epsilon < 1.0);
        let l = q_init.len();
        EpsilonGreedyAgent {
            q_star: q_init,
            stepper,
            epsilon,
            uniform: Uniform::new(0.0, 1.0),
            pick_arm: Uniform::new(0usize, l),
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{HarmonicStepper, Stepper};

    use super::{Agent, EpsilonGreedyAgent};

    lazy_static! {
        static ref Q_INIT: Vec<f64> = vec![0.5, 0.61, 0.7, 0.12, 0.37];
    }

    #[test]
    fn test_new() {
        let mut STEPPER = HarmonicStepper::new(1, Q_INIT.len());
        let epsilon = 0.1;
        let GREEDY: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), &mut STEPPER, epsilon);
        assert_eq!(GREEDY.epsilon, epsilon);
        assert_eq!(GREEDY.q_star, vec![0.5, 0.61, 0.7, 0.12, 0.37])
    }

    #[test]
    #[should_panic]
    fn test_new_big_epsilon() {
        let mut STEPPER = HarmonicStepper::new(1, Q_INIT.len());
        let epsilon = 1.3;
        let GREEDY: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), &mut STEPPER, epsilon);
    }

    #[test]
    #[should_panic]
    fn test_new_small_epsilon() {
        let mut STEPPER = HarmonicStepper::new(1, Q_INIT.len());
        let epsilon = -0.3;
        let GREEDY: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), &mut STEPPER, epsilon);
    }

    #[test]
    fn test_q_star() {
        let mut STEPPER = HarmonicStepper::new(1, Q_INIT.len());
        let GREEDY: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), &mut STEPPER, 0.1);
        assert_eq!(GREEDY.q_star(), &vec![0.5, 0.61, 0.7, 0.12, 0.37])
    }

    #[test]
    fn test_reset() {
        let mut STEPPER = HarmonicStepper::new(1, Q_INIT.len());
        let mut GREEDY: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), &mut STEPPER, 0.1);
        let new_q = vec![0.01, 0.86, 0.43, 0.65, 0.66];
        GREEDY.reset(new_q.clone());
        assert_eq!(GREEDY.q_star(), &new_q)
    }
}
