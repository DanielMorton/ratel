use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Binomial;

use crate::bandit::bandit::Bandit;

use super::ArgBounds;

struct BinomialBandit {
    nums: Vec<i32>,
    probs: Vec<f64>,
    distributions: Vec<Binomial>,
}

impl BinomialBandit {
    fn new(nums: Vec<i32>, probs: Vec<f64>) -> BinomialBandit {
        assert_eq!(nums.len(), probs.len());
        assert!(probs.val_max() <= 1.0);
        assert!(probs.val_min() >= 0.0);
        assert!(nums.val_min() > 0);
        let dist = nums
            .iter()
            .zip(&probs)
            .map(|(n, p)| Binomial::new(*n as u64, *p).unwrap())
            .collect();
        BinomialBandit {
            nums,
            probs,
            distributions: dist,
        }
    }
}

impl Bandit for BinomialBandit {
    fn means(&self) -> Vec<f64> {
        self.nums.iter().zip(&self.probs).map(|(n, p)| (*n as f64) * *p).collect()
    }

    fn reward(&self, arm: usize) -> f64 {
        self.distributions[arm].sample(&mut thread_rng()) as f64
    }

    fn stds(&self) -> Vec<f64> {
        self.nums.iter().zip(&self.probs).map(|(n, p)| (*n as f64) * *p * (1.0 - *p))
            .map(|s| s.sqrt()).collect()
    }
}
