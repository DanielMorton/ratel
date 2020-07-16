use num_traits::ToPrimitive;

/// A trait for common members of the Bandits
pub trait Bandit<T: ToPrimitive> {
    /// The number of arms of the Bandit
    fn arms(&self) -> usize;

    /// The arm with the highest average reward.
    fn best_arm(&self) -> usize;

    /// The maximum average reward of all the arms.
    fn max_reward(&self) -> f64 {
        self.mean(self.best_arm())
    }

    /// The average reward of a given arm.
    fn mean(&self, arm: usize) -> f64;

    /// The average rewards of all the arms.
    fn means(&self) -> Vec<f64> {
        (0..self.arms()).map(|arm| self.mean(arm)).collect()
    }

    /// The reward from a pull of a given arm.
    fn reward(&self, arm: usize) -> T;

    /// The standard deviation of a given arm.
    fn std(&self, arm: usize) -> f64;

    /// the standard deviations of all the arms.
    fn stds(&self) -> Vec<f64> {
        (0..self.arms()).map(|arm| self.std(arm)).collect()
    }
}
