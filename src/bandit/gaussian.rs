use rand::Rng;
use rand::thread_rng;
use rand_distr::Normal;

use super::ArgBounds;
use super::Bandit;
use super::Distributions;

struct GaussianBandit<R: Rng> {
    means: Vec<f64>,
    std: Vec<f64>,
    distributions: Distributions<Normal<f64>, R>,
}

impl<R: Rng> Bandit for GaussianBandit<R> {
    fn best_arm(&self) -> usize {
        self.means.arg_max()
    }

    fn max_reward(&self) -> f64 {
        self.means.val_max()
    }

    fn reward(&mut self, arm: usize) -> f64 {
        self.distributions.reward(arm)
    }
}
