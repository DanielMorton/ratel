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

impl Bandit<u64> for BinomialBandit {
    fn means(&self) -> Vec<f64> {
        self.nums
            .iter()
            .zip(&self.probs)
            .map(|(n, p)| (*n as f64) * *p)
            .collect()
    }

    fn reward(&self, arm: usize) -> u64 {
        self.distributions[arm].sample(&mut thread_rng())
    }

    fn stds(&self) -> Vec<f64> {
        self.nums
            .iter()
            .zip(&self.probs)
            .map(|(n, p)| (*n as f64) * *p * (1.0 - *p))
            .map(|s| s.sqrt())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use super::BinomialBandit;
    use super::super::Bandit;

    lazy_static! {
        static ref NUMS_VEC: Vec<i32> = vec![5, 4, 1, 8, 10];
        static ref PROBS_VEC: Vec<f64> = vec![0.97, 0.91, 0.77, 0.66, 0.57];
        static ref BINOM: BinomialBandit =
            BinomialBandit::new(NUMS_VEC.to_vec(), PROBS_VEC.to_vec());
    }

    #[test]
    fn test_arms() {
        assert_eq!(BINOM.arms(), 5)
    }

    #[test]
    fn test_best_arm() {
        assert_eq!(BINOM.best_arm(), 4)
    }

    #[test]
    fn test_max_reward() {
        assert_approx_eq!(BINOM.max_reward(), 5.7)
    }

    #[test]
    fn test_mean() {
        assert_eq!(BINOM.mean(1), 3.64)
    }

    #[test]
    fn test_means() {
        BINOM
            .means()
            .iter()
            .zip(vec![4.85, 3.64, 0.77, 5.28, 5.7])
            .for_each(|(m1, m2)| assert_approx_eq!(m1, m2))
    }

    #[test]
    #[should_panic]
    fn test_new_big_prob() {
        let p = vec![0.97, 0.91, 0.77, 0.66, 1.05];
        BinomialBandit::new(NUMS_VEC.to_vec(), p);
    }

    #[test]
    #[should_panic]
    fn test_new_neg_num() {
        let n = vec![5, 4, 1, -8, 10];
        BinomialBandit::new(n, PROBS_VEC.to_vec());
    }

    #[test]
    #[should_panic]
    fn test_new_neg_prob() {
        let p = vec![0.97, 0.91, 0.77, 0.66, -0.05];
        BinomialBandit::new(NUMS_VEC.to_vec(), p);
    }

    #[test]
    #[should_panic]
    fn test_new_wrong_size() {
        let p = vec![0.97, 0.91, 0.77, 0.66];
        BinomialBandit::new(NUMS_VEC.to_vec(), p);
    }

    #[test]
    fn test_reward() {
        for _ in 0..1000 {
            BINOM.reward(2);
        }
    }

    #[test]
    fn test_std() {
        assert_approx_eq!(BINOM.std(1), 0.5723635)
    }

    #[test]
    fn test_stds() {
        BINOM
            .stds()
            .iter()
            .zip(vec![
                0.38144462, 0.57236352, 0.42083251, 1.33985074, 1.56556699,
            ])
            .for_each(|(s1, s2)| assert_approx_eq!(s1, s2))
    }
}
