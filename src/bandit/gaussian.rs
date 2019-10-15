use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Normal;

use super::{ArgBounds, Bandit};

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

impl Bandit<f64> for GaussianBandit {
    fn means(&self) -> Vec<f64> {
        self.means.clone()
    }

    fn reward(&self, arm: &usize) -> f64 {
        self.distributions[*arm].sample(&mut thread_rng())
    }

    fn stds(&self) -> Vec<f64> {
        self.stds.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::GaussianBandit;
    use super::super::Bandit;

    lazy_static! {
        static ref MEAN_VEC: Vec<f64> = vec![-1.83, -0.82, -1.35, 2.61, 0.39];
        static ref STD_VEC: Vec<f64> = vec![2.3, 1.25, 0.78, 1.80, 1.55];
        static ref GAUSS: GaussianBandit = GaussianBandit::new(MEAN_VEC.to_vec(), STD_VEC.to_vec());
    }

    #[test]
    fn test_arms() {
        assert_eq!(GAUSS.arms(), 5)
    }

    #[test]
    fn test_best_arm() {
        assert_eq!(GAUSS.best_arm(), 3)
    }

    #[test]
    fn test_max_reward() {
        assert_eq!(GAUSS.max_reward(), 2.61)
    }

    #[test]
    fn test_mean() {
        assert_eq!(GAUSS.mean(1), -0.82)
    }

    #[test]
    fn test_means() {
        assert_eq!(GAUSS.means(), vec![-1.83, -0.82, -1.35, 2.61, 0.39])
    }

    #[test]
    #[should_panic]
    fn test_new_wrong_size() {
        let s = vec![2.3, 1.25, 0.78, 1.80];
        GaussianBandit::new(MEAN_VEC.to_vec(), s);
    }

    #[test]
    #[should_panic]
    fn test_new_neg_sd() {
        let s = vec![2.3, 1.25, 0.78, 1.80, -1.55];
        GaussianBandit::new(MEAN_VEC.to_vec(), s);
    }

    #[test]
    fn test_reward() {
        for _ in 0..1000 {
            GAUSS.reward(2);
        }
    }

    #[test]
    fn test_std() {
        assert_eq!(GAUSS.std(1), 1.25)
    }

    #[test]
    fn test_stds() {
        assert_eq!(GAUSS.stds(), vec![2.3, 1.25, 0.78, 1.80, 1.55])
    }
}
