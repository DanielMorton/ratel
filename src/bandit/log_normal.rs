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

    /// Number of arms on the bandit.
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

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::LogNormalBandit;
    use super::super::Bandit;

    #[test]
    fn test_arms() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let log_norm: LogNormalBandit = LogNormalBandit::new(&mus_vec, &sigmas_vec);
        assert_eq!(log_norm.arms(), 5)
    }

    #[test]
    fn test_best_arm() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let log_norm: LogNormalBandit = LogNormalBandit::new(&mus_vec, &sigmas_vec);
        assert_eq!(log_norm.best_arm(), 3)
    }

    #[test]
    fn test_max_reward() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let log_norm: LogNormalBandit = LogNormalBandit::new(&mus_vec, &sigmas_vec);
        assert_approx_eq!(log_norm.max_reward(), 68.71723217)
    }

    #[test]
    fn test_mean() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let log_norm: LogNormalBandit = LogNormalBandit::new(&mus_vec, &sigmas_vec);
        assert_approx_eq!(log_norm.mean(1), 0.96199118)
    }

    #[test]
    fn test_means() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let log_norm: LogNormalBandit = LogNormalBandit::new(&mus_vec, &sigmas_vec);
        log_norm
            .means()
            .iter()
            .zip(vec![
                2.25917567,
                0.96199118,
                0.35141058,
                68.71723217,
                4.90988245,
            ])
            .for_each(|(m1, m2)| assert_approx_eq!(m1, m2))
    }

    #[test]
    #[should_panic]
    fn test_new_wrong_size() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80];
        LogNormalBandit::new(&mus_vec, &sigmas_vec);
    }

    #[test]
    #[should_panic]
    fn test_new_neg_sigma() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, -1.80, 1.55];
        LogNormalBandit::new(&mus_vec, &sigmas_vec);
    }

    #[test]
    fn test_reward() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let log_norm: LogNormalBandit = LogNormalBandit::new(&mus_vec, &sigmas_vec);
        for _ in 0..1000 {
            log_norm.reward(2);
        }
    }

    #[test]
    fn test_std() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let log_norm: LogNormalBandit = LogNormalBandit::new(&mus_vec, &sigmas_vec);
        assert_approx_eq!(log_norm.std(1), 1.86803062)
    }

    #[test]
    fn test_stds() {
        let mus_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let sigmas_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let log_norm: LogNormalBandit = LogNormalBandit::new(&mus_vec, &sigmas_vec);
        log_norm
            .stds()
            .iter()
            .zip(vec![
                31.7366684,
                1.86803062,
                0.321591383,
                340.366945,
                15.5657745,
            ])
            .for_each(|(s1, s2)| assert_approx_eq!(s1, s2))
    }
}
