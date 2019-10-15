use super::ArgBounds;

pub trait Bandit<T> {
    fn arms(&self) -> usize {
        self.means().len()
    }

    fn best_arm(&self) -> usize {
        self.means().arg_max()
    }

    fn max_reward(&self) -> f64 {
        self.means().val_max()
    }

    fn mean(&self, arm: usize) -> f64 {
        self.means()[arm]
    }

    fn means(&self) -> Vec<f64>;

    fn reward(&self, arm: &usize) -> T;

    fn std(&self, arm: usize) -> f64 {
        self.stds()[arm]
    }

    fn stds(&self) -> Vec<f64>;
}
