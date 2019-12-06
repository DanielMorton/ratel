use std::marker::PhantomData;

use num_traits::ToPrimitive;

use crate::Stepper;

use super::{Agent, ArgBounds};

/// Agent that follows the Greedy Algorithm.
///
/// Always chooses the arm with the highest estimated reward.
pub struct GreedyAgent<'a, T> {
    /// The current estimates of the Bandit arm values.
    q_star: Vec<f64>,

    /// The Agent's rule for step size updates.
    stepper: &'a mut dyn Stepper,
    phantom: PhantomData<T>,
}

impl<'a, T: ToPrimitive> Agent<T> for GreedyAgent<'a, T> {
    /// The action chosen by the Agent. Picks the arm with the highest estimated return.
    fn action(&self) -> usize {
        self.q_star.arg_max()
    }

    /// The Agent's current estimate of all the Bandit's arms.
    fn q_star(&self) -> &Vec<f64> {
        &self.q_star
    }

    /// Reset the Agent's history and give it a new initial guess of the Bandit's arm values.
    fn reset(&mut self, q_init: Vec<f64>) {
        self.q_star = q_init;
        self.stepper.reset()
    }

    /// Update the Agent's estimate of a Bandit arm based on a given reward.
    fn step(&mut self, arm: usize, reward: T) -> () {
        self.q_star[arm] += self.update(arm, reward)
    }

    /// Returns a reference to the Agent's step size update rule.
    fn stepper(&mut self) -> &mut dyn Stepper {
        self.stepper
    }
}

impl<'a, T> GreedyAgent<'a, T> {
    /// Initializes a new Greedy agent.
    pub fn new(q_init: Vec<f64>, stepper: &'a mut dyn Stepper) -> GreedyAgent<'a, T> {
        GreedyAgent {
            q_star: q_init,
            stepper,
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{HarmonicStepper, Stepper};

    use super::{Agent, GreedyAgent};

    lazy_static! {
        static ref Q_INIT: Vec<f64> = vec![0.5, 0.61, 0.7, 0.12, 0.37];
    }

    #[test]
    fn test_action() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let greedy: GreedyAgent<u32> = GreedyAgent::new(Q_INIT.to_vec(), &mut stepper);
        assert_eq!(greedy.action(), 2)
    }

    #[test]
    fn test_q_star() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let greedy: GreedyAgent<u32> = GreedyAgent::new(Q_INIT.to_vec(), &mut stepper);
        assert_eq!(greedy.q_star(), &vec![0.5, 0.61, 0.7, 0.12, 0.37])
    }

    #[test]
    fn test_reset() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let mut greedy: GreedyAgent<u32> = GreedyAgent::new(Q_INIT.to_vec(), &mut stepper);
        let new_q = vec![0.01, 0.86, 0.43, 0.65, 0.66];
        greedy.reset(new_q.clone());
        assert_eq!(greedy.q_star(), &new_q)
    }
}
