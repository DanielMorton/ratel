use super::ArgBounds;

/// A trait for common members of the Bandits
pub trait Bandit<T> {
    /// The number of arms of the Bandit
    fn arms(&self) -> usize {
        self.means().len()
    }

    /// The arm with the highest average reward.
    fn best_arm(&self) -> usize {
        self.means().arg_max()
    }

    /// The maximum average reward of all the arms.
    fn max_reward(&self) -> f64 {
        self.means().val_max()
    }

    /// The average reward of a given arm.
    fn mean(&self, arm: usize) -> f64 {
        self.means()[arm]
    }

    /// The average rewards of all the arms.
    fn means(&self) -> Vec<f64>;

    /// The reward from a pull of a given arm.
    fn reward(&self, arm: usize) -> T;

    /// The standard deviation of a given arm.
    fn std(&self, arm: usize) -> f64 {
        self.stds()[arm]
    }

    /// the standard deviations of all the arms.
    fn stds(&self) -> Vec<f64>;
}
