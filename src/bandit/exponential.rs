use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Exp;

use super::{ArgBounds, Bandit};

/// A bandit whose arms distribute rewards according to the exponential distributions.
pub struct ExponentialBandit {
    /// Vector of the inverses of the distribution means.
    lambdas: Vec<f64>,

    /// Number of arms on the bandit.
    arms: usize,

    /// The bandit arm with highest reward.
    best_arm: usize,

    /// Distributions of the arms.
    distributions: Vec<Exp<f64>>,
}

impl ExponentialBandit {
    /// Initializes a new Bandit where each arm distributes rewards according to an exponential
    /// distribution.
    pub fn new(lambdas: Vec<f64>) -> ExponentialBandit {
        assert!(lambdas.val_min() > 0.0);
        let dist = lambdas.iter().map(|&l| Exp::new(l).unwrap()).collect();
        let arms = lambdas.len();
        let best_arm = lambdas.arg_min();
        ExponentialBandit {
            lambdas,
            arms,
            best_arm,
            distributions: dist,
        }
    }
}

impl Bandit<f64> for ExponentialBandit {
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

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::ExponentialBandit;
    use super::super::Bandit;

    #[test]
    fn test_arms() {
        let lambdas_vec: Vec<f64> = vec![6.1, 0.6, 5.4, 9.1, 2.5];
        let exp: ExponentialBandit = ExponentialBandit::new(lambdas_vec);
        assert_eq!(exp.arms(), 5)
    }

    #[test]
    fn test_best_arm() {
        let lambdas_vec: Vec<f64> = vec![6.1, 0.6, 5.4, 9.1, 3.5];
        let exp: ExponentialBandit = ExponentialBandit::new(lambdas_vec);
        assert_eq!(exp.best_arm(), 1)
    }

    #[test]
    fn test_max_reward() {
        let lambdas_vec: Vec<f64> = vec![6.1, 0.6, 5.4, 9.1, 3.5];
        let exp: ExponentialBandit = ExponentialBandit::new(lambdas_vec);
        assert_approx_eq!(exp.max_reward(), 1.6666667)
    }

    #[test]
    fn test_mean() {
        let lambdas_vec: Vec<f64> = vec![6.1, 0.6, 5.4, 9.1, 2.5];
        let exp: ExponentialBandit = ExponentialBandit::new(lambdas_vec);
        assert_eq!(exp.mean(4), 0.4)
    }

    #[test]
    fn test_means() {
        let lambdas_vec: Vec<f64> = vec![6.1, 0.6, 5.4, 9.1, 3.5];
        let exp: ExponentialBandit = ExponentialBandit::new(lambdas_vec);
        exp.means()
            .iter()
            .zip(vec![6.1, 0.6, 5.4, 9.1, 3.5])
            .for_each(|(m1, m2)| assert_approx_eq!(1.0 / m1, m2))
    }

    #[test]
    #[should_panic]
    fn test_new_neg_lambda() {
        let lambdas_vec: Vec<f64> = vec![6.1, 0.6, -5.4, 9.1, 3.5];
        ExponentialBandit::new(lambdas_vec);
    }

    #[test]
    fn test_reward() {
        let lambdas_vec: Vec<f64> = vec![6.1, 0.6, 5.4, 9.1, 3.5];
        let exp: ExponentialBandit = ExponentialBandit::new(lambdas_vec);
        for _ in 0..1000 {
            exp.reward(2);
        }
    }

    #[test]
    fn test_std() {
        let lambdas_vec: Vec<f64> = vec![6.1, 0.6, 5.4, 9.1, 3.5];
        let exp: ExponentialBandit = ExponentialBandit::new(lambdas_vec);
        assert_approx_eq!(exp.std(1), 1.6666667)
    }

    #[test]
    fn test_stds() {
        let lambdas_vec: Vec<f64> = vec![6.1, 0.6, 5.4, 9.1, 3.5];
        let exp: ExponentialBandit = ExponentialBandit::new(lambdas_vec);
        exp.stds()
            .iter()
            .zip(vec![6.1, 0.6, 5.4, 9.1, 3.5])
            .for_each(|(s1, s2)| assert_approx_eq!(1.0 / s1, s2))
    }
}
