use num_traits::ToPrimitive;

use crate::Stepper;

/// A trait for common members of the Agents.
pub trait Agent<T: ToPrimitive> {
    /// The action chosen by the Agent.
    fn action(&self) -> usize;

    /// The number of arms in the Bandit the Agent is playing.
    fn arms(&self) -> usize;

    /// The Agent's current estimate of the value of a Bandit's arm.
    fn current_estimate(&self, arm: usize) -> f64;

    /// Reset the Agent's history and give it a new initial guess of the Bandit's arm values.
    fn reset(&mut self, q_init: Vec<f64>);

    /// Update the Agent's estimate of a Bandit arm based on a given reward.
    fn step(&mut self, arm: usize, reward: T);
}

/// Calculate the update of the Agent's guess of a Bandit arm based on a given reward.
pub(crate) fn update<T: ToPrimitive>(
    stepper: &mut Box<dyn Stepper>,
    q_star: &[f64],
    arm: usize,
    reward: T,
) -> f64 {
    stepper.step(arm) * (reward.to_f64().unwrap() - q_star[arm])
}
