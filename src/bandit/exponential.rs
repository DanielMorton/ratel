use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Exp;

use super::{ArgBounds, Bandit};

/// A bandit whose arms distribute rewards according to the exponential distributions.
pub struct ExponentialBandit<'a> {
    /// Vector of the inverses of the distribution means.
    lambdas: &'a Vec<f64>,

    arms: usize,

    /// The bandit arm with highest reward.
    best_arm: usize,

    /// Distributions of the arms.
    distributions: Vec<Exp<f64>>,
}

impl<'a> ExponentialBandit<'a> {
    /// Initializes a new Bandit where each arm distributes rewards according to a binomial
    /// distribution.
    pub fn new(lambdas: &'a Vec<f64>) -> ExponentialBandit<'a> {
        assert!(lambdas.val_min() > 0.0);
        let dist = lambdas.iter().map(|&l| Exp::new(l).unwrap()).collect();
        ExponentialBandit {
            lambdas,
            arms: lambdas.len(),
            best_arm: lambdas.arg_min(),
            distributions: dist,
        }
    }
}

impl<'a> Bandit<f64> for ExponentialBandit<'a> {
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
        1.0 / self.lambdas[arm]
    }

    /// Determines the reward for pulling a given arm.
    fn reward(&self, arm: usize) -> f64 {
        self.distributions[arm].sample(&mut thread_rng())
    }

    /// The standard deviations of each arm.
    fn std(&self, arm: usize) -> f64 {
        1.0 / self.lambdas[arm]
    }
}
