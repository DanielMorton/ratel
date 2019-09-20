use crate::util::Stepper;

use super::Bandit;

pub(super) trait Agent<T> {
    fn action(&self) -> usize;

    fn arms(&self) -> usize {
        self.bandit().arms()
    }

    fn bandit(&self) -> &dyn Bandit<T>;

    fn best_arm(&self) -> usize {
        self.bandit().best_arm()
    }

    fn current_estimate(&self) -> f64;

    fn max_reward(&self) -> f64 {
        self.bandit().max_reward()
    }

    fn reward(&self, action: usize) -> T {
        self.bandit().reward(action)
    }

    fn step(&mut self, action: usize) -> f64 {
        self.stepper(action).step()
    }

    fn stepper(&self, action: usize) -> &mut Stepper;
}
