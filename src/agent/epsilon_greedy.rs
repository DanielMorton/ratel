use std::marker::PhantomData;

use num_traits::ToPrimitive;
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::uniform::Uniform;

use crate::agent::agent::update;
use crate::Stepper;

use super::{Agent, ArgBounds};

/// Agent that follows the Epsilon-Greedy Algorithm.
///
/// A fixed (usually small) percentage of the
/// time it picks a random arm; the rest of the time it picks the arm with the highest expected
/// reward.
pub struct EpsilonGreedyAgent<T> {
    /// The current estimates of the Bandit arm values.
    q_star: Vec<f64>,

    /// The Agent's rule for step size updates.
    stepper: Box<dyn Stepper>,

    /// The fraction of times a random arm is chosen.
    epsilon: f64,

    /// A random uniform distribution to determine if a random action should be chosen.
    uniform: Uniform<f64>,

    /// A random uniform distribution to chose a random arm.
    pick_arm: Uniform<usize>,
    phantom: PhantomData<T>,
}

impl<T: ToPrimitive> Agent<T> for EpsilonGreedyAgent<T> {
    /// The action chosen by the Agent. A random action with probability `epsilon` and the greedy
    /// action otherwise.
    fn action(&self) -> usize {
        if self.uniform.sample(&mut thread_rng()) < self.epsilon {
            self.pick_arm.sample(&mut thread_rng())
        } else {
            self.q_star.arg_max()
        }
    }

    /// The number of arms in the Bandit the Agent is playing.
    fn arms(&self) -> usize {
        self.q_star.len()
    }

    /// The Agent's current estimate of the value of a Bandit's arm.
    fn current_estimate(&self, arm: usize) -> f64 {
        self.q_star[arm]
    }

    /// Reset the Agent's history and give it a new initial guess of the Bandit's arm values.
    fn reset(&mut self, q_init: Vec<f64>) {
        self.q_star = q_init;
        self.stepper.reset()
    }

    /// Update the Agent's estimate of a Bandit arm based on a given reward.
    fn step(&mut self, arm: usize, reward: T) {
        self.q_star[arm] += update(&mut self.stepper, &self.q_star, arm, reward)
    }
}

impl<T> EpsilonGreedyAgent<T> {
    /// Initializes a new Epsilon-Greedy agent.
    pub fn new(q_init: Vec<f64>, stepper: Box<dyn Stepper>, epsilon: f64) -> EpsilonGreedyAgent<T> {
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
    use crate::HarmonicStepper;

    use super::{Agent, EpsilonGreedyAgent};

    lazy_static! {
        static ref Q_INIT: Vec<f64> = vec![0.5, 0.61, 0.7, 0.12, 0.37];
    }

    #[test]
    fn test_new() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let eps = 0.1;
        let epsilon: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), Box::new(stepper), eps);
        assert_eq!(epsilon.epsilon, eps);
        assert_eq!(epsilon.q_star, vec![0.5, 0.61, 0.7, 0.12, 0.37])
    }

    #[test]
    #[should_panic]
    fn test_new_big_epsilon() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let eps = 1.3;
        let epsilon: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), Box::new(stepper), eps);
    }

    #[test]
    #[should_panic]
    fn test_new_small_epsilon() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let eps = -0.3;
        let epsilon: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), Box::new(stepper), eps);
    }

    #[test]
    fn test_q_star() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let epsilon: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), Box::new(stepper), 0.1);
        assert_eq!(epsilon.q_star(), &vec![0.5, 0.61, 0.7, 0.12, 0.37])
    }

    #[test]
    fn test_reset() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let mut epsilon: EpsilonGreedyAgent<u32> =
            EpsilonGreedyAgent::new(Q_INIT.to_vec(), Box::new(stepper), 0.1);
        let new_q = vec![0.01, 0.86, 0.43, 0.65, 0.66];
        epsilon.reset(new_q.clone());
        assert_eq!(epsilon.q_star(), &new_q)
    }
}
