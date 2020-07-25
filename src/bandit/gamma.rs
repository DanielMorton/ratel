use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Gamma;

use super::{ArgBounds, Bandit};

/// A bandit whose arms distribute rewards according to the exponential distributions.
pub struct GammaBandit<'a> {
    /// Vector of distribution shape parameters.
    alphas: &'a Vec<f64>,

    /// Vector of distribution scale parameters.
    thetas: &'a Vec<f64>,

    arms: usize,

    /// The bandit arm with highest reward.
    best_arm: usize,

    /// Distributions of the arms.
    distributions: Vec<Gamma<f64>>,
}

impl<'a> GammaBandit<'a> {
    /// Initializes a new Bandit where each arm distributes rewards according to a binomial
    /// distribution.
    fn new(alphas: &'a Vec<f64>, thetas: &'a Vec<f64>) -> GammaBandit<'a> {
        assert_eq!(alphas.len(), thetas.len());
        assert!(alphas.val_min() > 0.0);
        assert!(thetas.val_min() > 0.0);
        let dist = alphas
            .iter()
            .zip(thetas)
            .map(|(&a, &t)| Gamma::new(a, t).unwrap())
            .collect();
        let best_arm = alphas
            .iter()
            .zip(thetas)
            .map(|(&a, &t)| a * t).collect::<Vec<f64>>().arg_max();
        GammaBandit {
            alphas,
            thetas,
            arms: thetas.len(),
            best_arm,
            distributions: dist,
        }
    }
}

impl<'a> Bandit<f64> for GammaBandit<'a> {
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
        self.alphas[arm] * self.thetas[arm]
    }

    /// Determines the reward for pulling a given arm.
    fn reward(&self, arm: usize) -> f64 {
        self.distributions[arm].sample(&mut thread_rng())
    }

    /// The standard deviations of each arm.
    fn std(&self, arm: usize) -> f64 {
        (self.alphas[arm] * self.thetas[arm] * self.thetas[arm]).sqrt()
    }
}