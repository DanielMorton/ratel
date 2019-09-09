pub(super) trait Bandit {
    fn arms(&self) -> usize;

    fn best_arm(&self) -> usize;

    fn max_reward(&self) -> f64;

    fn reward(&self, arm: usize) -> f64;
}

/*#[cfg(test)]
mod tests {
    use super::Bandit;

    lazy_static! {
        static ref BANDIT: Bandit = Bandit {
            q_values: vec![0.46, 0.61, 0.19, 0.5, 0.99]
        };
    }

    #[test]
    fn test_arms() {
        assert_eq!(BANDIT.arms(), 5)
    }

    #[test]
    fn test_best_arm() {
        assert_eq!(BANDIT.best_arm(), 4)
    }

    #[test]
    fn test_max_reward() {
        assert_eq!(BANDIT.max_reward(), 0.99)
    }
}*/
