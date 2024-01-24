use std::marker::PhantomData;

use num_traits::ToPrimitive;

use crate::agent::agent::update;
use crate::Stepper;

use super::{Agent, ArgBounds};

/// Agent that follows the Optimistic Algorithm.
///
/// Always chooses the arm with the highest confidence bound.
pub struct OptimisticAgent<T> {
    /// The current estimates of the Bandit arm values.
    q_star: Vec<f64>,

    /// Confidence bound.
    c: f64,

    /// Total number of rounds the Agent has played.
    total: f64,

    /// Total number of times the Agent has played each arm.
    arm_total: Vec<f64>,

    /// The Agent's rule for step size updates.
    stepper: Box<dyn Stepper>,
    phantom: PhantomData<T>,
}

impl<T: ToPrimitive> Agent<T> for OptimisticAgent<T> {
    /// The action chosen by the Agent. The agent chooses the action with the highest confidence
    /// bound.
    fn action(&self) -> usize {
        self.q_star
            .iter()
            .zip(&self.arm_total)
            .map(|(&q, &c)| q + self.c * (self.total.ln() / c).sqrt())
            .collect::<Vec<f64>>()
            .arg_max()
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

    /// Update the Agent's totals and estimate of a Bandit arm based on a given reward.
    fn step(&mut self, arm: usize, reward: T) {
        self.q_star[arm] += update(&mut self.stepper, &self.q_star, arm, reward);
        self.arm_total[arm] += 1.0;
        self.total += 1.0
    }
}

impl<T> OptimisticAgent<T> {
    /// Initializes a new Optimistic agent.
    pub fn new(q_init: Vec<f64>, c: f64, stepper: Box<dyn Stepper>) -> OptimisticAgent<T> {
        assert!(c > 0.0);
        OptimisticAgent {
            arm_total: vec![1.0; q_init.len()],
            q_star: q_init,
            c,
            total: 1.0,
            stepper,
            phantom: PhantomData,
        }
    }

    /// Returns a reference to the current number of times each arm has been pulled.
    fn arm_total(&self) -> &Vec<f64> {
        &self.arm_total
    }
}

#[cfg(test)]
mod tests {
    use crate::HarmonicStepper;

    use super::{Agent, OptimisticAgent};

    #[test]
    fn test_new() {
        let Q_INIT = vec![0.5, 0.61, 0.7, 0.12, 0.37];
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let c = 2.0;
        let optimistic: OptimisticAgent<u32> =
            OptimisticAgent::new(Q_INIT, c, Box::new(stepper));
        assert_eq!(optimistic.c, c);
        assert_eq!(optimistic.q_star, vec![0.5, 0.61, 0.7, 0.12, 0.37])
    }

    #[test]
    fn test_action() {
        let Q_INIT = vec![0.5, 0.61, 0.7, 0.12, 0.37];
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let c = 2.0;
        let optimistic: OptimisticAgent<u32> =
            OptimisticAgent::new(Q_INIT, c, Box::new(stepper));
        assert_eq!(optimistic.action(), 2)
    }

    #[test]
    fn test_q_star() {
        let Q_INIT = vec![0.5, 0.61, 0.7, 0.12, 0.37];
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let c = 2.0;
        let optimistic: OptimisticAgent<u32> =
            OptimisticAgent::new(Q_INIT, c, Box::new(stepper));
        assert_eq!(optimistic.q_star, vec![0.5, 0.61, 0.7, 0.12, 0.37])
    }

    #[test]
    fn test_reset() {
        let Q_INIT = vec![0.5, 0.61, 0.7, 0.12, 0.37];
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let c = 2.0;
        let mut optimistic: OptimisticAgent<u32> =
            OptimisticAgent::new(Q_INIT, c, Box::new(stepper));
        let new_q = vec![0.01, 0.86, 0.43, 0.65, 0.66];
        optimistic.reset(new_q.clone());
        assert_eq!(optimistic.q_star, new_q)
    }
}
