use num_traits::ToPrimitive;

use crate::Stepper;

/// A trait for common members of the Agents.
pub trait Agent<T: ToPrimitive> {
    /// The action chosen by the Agent.
    fn action(&self) -> usize;

    /// The number of arms in the Bandit the Agent is playing.
    fn arms(&self) -> usize {
        self.q_star().len()
    }

    /// The Agent's current estimate of the value of a Bandit's arm.
    fn current_estimate(&self, arm: usize) -> f64 {
        self.q_star()[arm]
    }

    /// The Agent's current estimate of all the Bandit's arms.
    fn q_star(&self) -> &Vec<f64>;

    /// Reset the Agent's history and give it a new initial guess of the Bandit's arm values.
    fn reset(&mut self, q_init: Vec<f64>);

    /// Update the Agent's estimate of a Bandit arm based on a given reward.
    fn step(&mut self, arm: usize, reward: T);

    /// Calculate the update of the Agent's guess of a Bandit arm based on a given reward.
    fn update(&mut self, arm: usize, reward: T) -> f64 {
        self.stepper().step(arm) * (reward.to_f64().unwrap() - self.q_star()[arm])
    }

    /// Returns a reference to the Agent's step size update rule.
    fn stepper(&mut self) -> &mut dyn Stepper;
}
