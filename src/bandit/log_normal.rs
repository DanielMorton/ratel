use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::LogNormal;

use super::{ArgBounds, Bandit};

/// A bandit whose arms distribute rewards according to the Log Normal distributions.
pub struct LogNormalBandit<'a> {
    /// Means base normal distributions
    mus: &'a Vec<f64>,

    /// Standard deviations of the base normal distributions
    sigmas: &'a Vec<f64>,

    arms: usize,

    /// The bandit arm with highest reward.
    best_arm: usize,

    /// Distributions of the arms.
    distributions: Vec<LogNormal<f64>>,
}

impl<'a> LogNormalBandit<'a> {
    /// Initializes a new Bandit where each arm distributes rewards according to a Log Normal
    /// distribution.
    fn new(mus: &'a Vec<f64>, sigmas: &'a Vec<f64>) -> LogNormalBandit<'a> {
        assert_eq!(mus.len(), sigmas.len());
        assert!(sigmas.val_min() > 0.0);
        let dist = mus
            .iter()
            .zip(sigmas)
            .map(|(&m, &s)| LogNormal::new(m, s).unwrap())
            .collect();
        let best_arm = mus
            .iter()
            .zip(sigmas)
            .map(|(&m, &s)| m + s * s / 2.0)
            .collect::<Vec<f64>>()
            .arg_max();
        LogNormalBandit {
            mus,
            sigmas,
            arms: mus.len(),
            best_arm,
            distributions: dist,
        }
    }
}

impl<'a> Bandit<f64> for LogNormalBandit<'a> {
    ///Returns the number of arms on the bandit.
    fn arms(&self) -> usize {
        self.arms
    }

    ///Returns the arm with highest average reward.
    fn best_arm(&self) -> usize {
        self.best_arm
    }

    /// The expected return of each arm.
    fn mean(&self, arm: usize) -> f64 {
        (self.mus[arm] + self.sigmas[arm] * self.sigmas[arm] / 2.0).exp()
    }

    /// Determines the reward for pulling a given arm.
    fn reward(&self, arm: usize) -> f64 {
        self.distributions[arm].sample(&mut thread_rng())
    }

    /// The standard deviations of each arm.
    fn std(&self, arm: usize) -> f64 {
        (((self.sigmas[arm] * self.sigmas[arm]).exp() - 1.0)
            * (2.0 * self.mus[arm] + self.sigmas[arm] * self.sigmas[arm]).exp())
            .sqrt()
    }
}
