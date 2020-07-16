use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::Binomial;

use super::{ArgBounds, Bandit};

/// A bandit whose arms distribute rewards according to binomial distributions.
pub struct BinomialBandit<'a> {
    /// Vector of number of trials of a `yes-no` experiment.
    nums: &'a Vec<u32>,

    /// Vector of experiment success probabilities.
    probs: &'a Vec<f64>,

    /// The bandit arm with highest reward.
    best_arm: usize,

    /// Distributions of the arms.
    distributions: Vec<Binomial>,
}

impl<'a> BinomialBandit<'a> {
    /// Initializes a new Bandit where each arm distributes rewards according to a binomial
    /// distribution.
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
        let best_arm = nums.iter()
            .zip(probs)
            .map(|(&n, &p)| f64::from(n) * p).collect::<Vec<f64>>().arg_max();
        BinomialBandit {
            nums,
            probs,
            best_arm,
            distributions: dist,
        }
    }
}

impl<'a> Bandit<u32> for BinomialBandit<'a> {
    ///Returns the number of arms on the bandit.
    fn arms(&self) -> usize { self.nums.len() }

    ///Returns the arm with highest average reward.
    fn best_arm(&self) -> usize { self.best_arm }

    /// Computes the expected return of each arm.
    fn mean(&self, arm: usize) -> f64 {
        f64::from(self.nums[arm]) * self.probs[arm]
    }

    /// Determines the reward for pulling a given arm.
    fn reward(&self, arm: usize) -> u32 {
        self.distributions[arm].sample(&mut thread_rng()) as u32
    }

    /// Computes the standard deviations of each arm.
    fn std(&self, arm: usize) -> f64 {
        (f64::from(self.nums[arm]) * self.probs[arm] * (1.0 - self.probs[arm])).sqrt()
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
        print!("Run Test");
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
