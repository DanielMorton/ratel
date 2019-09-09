use rand::thread_rng;
use rand_distr::{Distribution, Normal};

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
        self.means.val_max()
    }

    fn reward(&self, arm: usize) -> f64 {
        Normal::new(self.means[arm], self.stddevs[arm])
            .unwrap()
            .sample(&mut thread_rng())
    }
}
