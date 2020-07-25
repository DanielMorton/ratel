use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Gamma;

use super::{ArgBounds, Bandit};

/// A bandit whose arms distribute rewards according to the gamma distributions.
pub struct GammaBandit<'a> {
    /// Vector of distribution shape parameters.
    alphas: &'a Vec<f64>,

    /// Vector of distribution scale parameters.
    thetas: &'a Vec<f64>,

    /// Number of arms on the bandit.
    arms: usize,

    /// The bandit arm with highest reward.
    best_arm: usize,

    /// Distributions of the arms.
    distributions: Vec<Gamma<f64>>,
}

impl<'a> GammaBandit<'a> {
    /// Initializes a new Bandit where each arm distributes rewards according to a gamma
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
            .map(|(&a, &t)| a * t)
            .collect::<Vec<f64>>()
            .arg_max();
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
        self.alphas[arm].sqrt() * self.thetas[arm]
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::GammaBandit;
    use super::super::Bandit;

    #[test]
    fn test_arms() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5, 5.9];
        let gamma: GammaBandit = GammaBandit::new(&alphas_vec, &thetas_vec);
        assert_eq!(gamma.arms(), 5)
    }

    #[test]
    fn test_best_arm() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5, 5.9];
        let gamma: GammaBandit = GammaBandit::new(&alphas_vec, &thetas_vec);
        assert_eq!(gamma.best_arm(), 4)
    }

    #[test]
    fn test_max_reward() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5, 5.9];
        let gamma: GammaBandit = GammaBandit::new(&alphas_vec, &thetas_vec);
        assert_approx_eq!(gamma.max_reward(), 52.51)
    }

    #[test]
    fn test_mean() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5, 5.9];
        let gamma: GammaBandit = GammaBandit::new(&alphas_vec, &thetas_vec);
        assert_approx_eq!(gamma.mean(1), 6.65)
    }

    #[test]
    fn test_means() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5, 5.9];
        let gamma: GammaBandit = GammaBandit::new(&alphas_vec, &thetas_vec);
        gamma
            .means()
            .iter()
            .zip(vec![10.4, 6.65, 5.28, 0.95, 52.51])
            .for_each(|(m1, m2)| assert_approx_eq!(m1, m2))
    }

    #[test]
    #[should_panic]
    fn test_new_neg_alphas() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, -1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5, 5.9];
        GammaBandit::new(&alphas_vec, &thetas_vec);
    }

    #[test]
    #[should_panic]
    fn test_new_neg_thetas() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, -0.7, 3.3, 0.5, 5.9];
        GammaBandit::new(&alphas_vec, &thetas_vec);
    }

    #[test]
    #[should_panic]
    fn test_new_wrong_size() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5];
        GammaBandit::new(&alphas_vec, &thetas_vec);
    }

    #[test]
    fn test_reward() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5, 5.9];
        let gamma: GammaBandit = GammaBandit::new(&alphas_vec, &thetas_vec);
        for _ in 0..1000 {
            gamma.reward(2);
        }
    }

    #[test]
    fn test_std() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5, 5.9];
        let gamma: GammaBandit = GammaBandit::new(&alphas_vec, &thetas_vec);
        assert_approx_eq!(gamma.std(1), 2.1575449)
    }

    #[test]
    fn test_stds() {
        let alphas_vec: Vec<f64> = vec![1.3, 9.5, 1.6, 1.9, 8.9];
        let thetas_vec: Vec<f64> = vec![8.0, 0.7, 3.3, 0.5, 5.9];
        let gamma: GammaBandit = GammaBandit::new(&alphas_vec, &thetas_vec);
        gamma
            .stds()
            .iter()
            .zip(vec![
                9.1214034,
                2.1575449,
                4.1742065,
                0.68920243,
                17.60139199,
            ])
            .for_each(|(s1, s2)| assert_approx_eq!(s1, s2))
    }
}
