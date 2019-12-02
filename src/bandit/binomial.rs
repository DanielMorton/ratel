use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Binomial;

use super::{ArgBounds, Bandit};

pub struct BinomialBandit<'a> {
    nums: &'a Vec<u32>,
    probs: &'a Vec<f64>,
    distributions: Vec<Binomial>,
}

impl<'a> BinomialBandit<'a> {
    pub fn new(nums: &'a Vec<u32>, probs: &'a Vec<f64>) -> BinomialBandit<'a> {
        assert_eq!(nums.len(), probs.len());
        assert!(probs.val_max() <= 1.0);
        assert!(probs.val_min() >= 0.0);
        assert!(nums.val_min() > 0);
        let dist = nums
            .iter()
            .zip(probs)
            .map(|(&n, &p)| Binomial::new(u64::from(n), p).unwrap())
            .collect();
        BinomialBandit {
            nums,
            probs,
            distributions: dist,
        }
    }
}

impl<'a> Bandit<u32> for BinomialBandit<'a> {
    fn means(&self) -> Vec<f64> {
        self.nums
            .iter()
            .zip(self.probs)
            .map(|(&n, &p)| f64::from(n) * p)
            .collect()
    }

    fn reward(&self, arm: usize) -> u32 {
        self.distributions[arm].sample(&mut thread_rng()) as u32
    }

    fn stds(&self) -> Vec<f64> {
        self.nums
            .iter()
            .zip(self.probs)
            .map(|(&n, &p)| f64::from(n) * p * (1.0 - p))
            .map(|s| s.sqrt())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::BinomialBandit;
    use super::super::Bandit;

    #[test]
    fn test_arms() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let probs_vec: Vec<f64> = vec![0.97, 0.91, 0.77, 0.66, 0.57];
        let binom: BinomialBandit = BinomialBandit::new(&nums_vec, &probs_vec);
        assert_eq!(binom.arms(), 5)
    }

    #[test]
    fn test_best_arm() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let probs_vec: Vec<f64> = vec![0.97, 0.91, 0.77, 0.66, 0.57];
        let binom: BinomialBandit = BinomialBandit::new(&nums_vec, &probs_vec);
        assert_eq!(binom.best_arm(), 4)
    }

    #[test]
    fn test_max_reward() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let probs_vec: Vec<f64> = vec![0.97, 0.91, 0.77, 0.66, 0.57];
        let binom: BinomialBandit = BinomialBandit::new(&nums_vec, &probs_vec);
        assert_approx_eq!(binom.max_reward(), 5.7)
    }

    #[test]
    fn test_mean() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let probs_vec: Vec<f64> = vec![0.97, 0.91, 0.77, 0.66, 0.57];
        let binom: BinomialBandit = BinomialBandit::new(&nums_vec, &probs_vec);
        assert_eq!(binom.mean(1), 3.64)
    }

    #[test]
    fn test_means() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let probs_vec: Vec<f64> = vec![0.97, 0.91, 0.77, 0.66, 0.57];
        let binom: BinomialBandit = BinomialBandit::new(&nums_vec, &probs_vec);
        binom
            .means()
            .iter()
            .zip(vec![4.85, 3.64, 0.77, 5.28, 5.7])
            .for_each(|(m1, m2)| assert_approx_eq!(m1, m2))
    }

    #[test]
    #[should_panic]
    fn test_new_big_prob() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let p = vec![0.97, 0.91, 0.77, 0.66, 1.05];
        BinomialBandit::new(&nums_vec, &p);
    }

    #[test]
    #[should_panic]
    fn test_new_neg_prob() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let p = vec![0.97, 0.91, 0.77, 0.66, -0.05];
        BinomialBandit::new(&nums_vec, &p);
    }

    #[test]
    #[should_panic]
    fn test_new_wrong_size() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let p = vec![0.97, 0.91, 0.77, 0.66];
        BinomialBandit::new(&nums_vec, &p);
    }

    #[test]
    fn test_reward() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let probs_vec: Vec<f64> = vec![0.97, 0.91, 0.77, 0.66, 0.57];
        let binom: BinomialBandit = BinomialBandit::new(&nums_vec, &probs_vec);
        for _ in 0..1000 {
            binom.reward(2);
        }
    }

    #[test]
    fn test_std() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let probs_vec: Vec<f64> = vec![0.97, 0.91, 0.77, 0.66, 0.57];
        let binom: BinomialBandit = BinomialBandit::new(&nums_vec, &probs_vec);
        assert_approx_eq!(binom.std(1), 0.5723635)
    }

    #[test]
    fn test_stds() {
        let nums_vec: Vec<u32> = vec![5, 4, 1, 8, 10];
        let probs_vec: Vec<f64> = vec![0.97, 0.91, 0.77, 0.66, 0.57];
        let binom: BinomialBandit = BinomialBandit::new(&nums_vec, &probs_vec);
        binom
            .stds()
            .iter()
            .zip(vec![
                0.38144462, 0.57236352, 0.42083251, 1.33985074, 1.56556699,
            ])
            .for_each(|(s1, s2)| assert_approx_eq!(s1, s2))
    }
}
