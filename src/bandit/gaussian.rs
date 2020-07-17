use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;

use super::{ArgBounds, Bandit};

/// A bandit whose arms distribute rewards according to Guassian distributions.
pub struct GaussianBandit<'a> {
    /// Means of the arms.
    means: &'a Vec<f64>,

    /// Standard deviations of the arms.
    stds: &'a Vec<f64>,

    /// The bandit arm with highest reward.
    best_arm: usize,

    /// Distributions of the arms.
    distributions: Vec<Normal<f64>>,
}

impl<'a> GaussianBandit<'a> {
    /// Initializes a new Bandit where each arm distributes rewards according to a Gaussian
    /// distribution.
    fn new(means: &'a Vec<f64>, stds: &'a Vec<f64>) -> GaussianBandit<'a> {
        assert_eq!(means.len(), stds.len());
        assert!(stds.val_min() > 0.0);
        let dist = means
            .iter()
            .zip(stds)
            .map(|(&m, &s)| Normal::new(m, s).unwrap())
            .collect();
        GaussianBandit {
            means,
            stds,
            best_arm: means.arg_max(),
            distributions: dist,
        }
    }
}

impl<'a> Bandit<f64> for GaussianBandit<'a> {
    ///Returns the number of arms on the bandit.
    fn arms(&self) -> usize {
        self.means.len()
    }

    ///Returns the arm with highest average reward.
    fn best_arm(&self) -> usize {
        self.best_arm
    }

    /// The expected return of each arm.
    fn mean(&self, arm: usize) -> f64 {
        self.means[arm]
    }

    /// Determines the reward for pulling a given arm.
    fn reward(&self, arm: usize) -> f64 {
        self.distributions[arm].sample(&mut thread_rng())
    }

    /// The standard deviations of each arm.
    fn std(&self, arm: usize) -> f64 {
        self.stds[arm]
    }
}

#[cfg(test)]
mod tests {
    use super::GaussianBandit;
    use super::super::Bandit;

    #[test]
    fn test_arms() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let std_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let gauss: GaussianBandit = GaussianBandit::new(&mean_vec, &std_vec);
        assert_eq!(gauss.arms(), 5)
    }

    #[test]
    fn test_best_arm() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let std_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let gauss: GaussianBandit = GaussianBandit::new(&mean_vec, &std_vec);
        assert_eq!(gauss.best_arm(), 3)
    }

    #[test]
    fn test_max_reward() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let std_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let gauss: GaussianBandit = GaussianBandit::new(&mean_vec, &std_vec);
        assert_eq!(gauss.max_reward(), 2.61)
    }

    #[test]
    fn test_mean() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let std_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let gauss: GaussianBandit = GaussianBandit::new(&mean_vec, &std_vec);
        assert_eq!(gauss.mean(1), -0.82)
    }

    #[test]
    fn test_means() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let std_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let gauss: GaussianBandit = GaussianBandit::new(&mean_vec, &std_vec);
        assert_eq!(gauss.means(), vec![-1.83, -0.82, -1.35, 2.61, 0.39])
    }

    #[test]
    #[should_panic]
    fn test_new_wrong_size() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let s = vec![2.3, 1.25, 0.78, 1.80];
        GaussianBandit::new(&mean_vec, &s);
    }

    #[test]
    #[should_panic]
    fn test_new_neg_sd() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let s = vec![2.3, 1.25, 0.78, 1.80, -1.55];
        GaussianBandit::new(&mean_vec, &s);
    }

    #[test]
    fn test_reward() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let std_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let gauss: GaussianBandit = GaussianBandit::new(&mean_vec, &std_vec);
        for _ in 0..1000 {
            gauss.reward(2);
        }
    }

    #[test]
    fn test_std() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let std_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let gauss: GaussianBandit = GaussianBandit::new(&mean_vec, &std_vec);
        assert_eq!(gauss.std(1), 1.25)
    }

    #[test]
    fn test_stds() {
        let mean_vec: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        let std_vec: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        let gauss: GaussianBandit = GaussianBandit::new(&mean_vec, &std_vec);
        assert_eq!(gauss.stds(), vec![2.3, 1.25, 0.78, 1.80, 1.55])
    }
}
