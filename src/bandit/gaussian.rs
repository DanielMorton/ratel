use super::ArgBounds;
use super::Bandit;

struct GaussianBandit {
    means: Vec<f64>,
    stddevs: Vec<f64>,
}

impl Bandit for GaussianBandit {
    fn arms(&self) -> usize {
        self.means.len()
    }

    fn best_arm(&self) -> usize {
        self.means.arg_max()
    }

    fn max_reward(&self) -> f64 {
        self.means.max()
    }
}