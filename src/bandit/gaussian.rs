use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;

use super::ArgBounds;
use super::Bandit;

//use super::Distributions;

struct GaussianBandit {
    means: Vec<f64>,
    stds: Vec<f64>,
    distributions: Vec<Normal<f64>>,
}

impl GaussianBandit {
    fn new(means: Vec<f64>, stds: Vec<f64>) -> GaussianBandit {
        assert_eq!(means.len(), stds.len());
        assert!(stds.val_min() > 0.0);
        let dist = means
            .iter()
            .zip(&stds)
            .map(|(m, s)| Normal::new(*m, *s).unwrap())
            .collect();
        GaussianBandit {
            means,
            stds,
            distributions: dist,
        }
    }
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

    fn reward(&mut self, arm: usize) -> f64 {
        self.distributions[arm].sample(&mut thread_rng())
    }
}
