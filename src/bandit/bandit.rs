pub(super) trait Bandit {
    fn arms(&self) -> usize;

    fn best_arm(&self) -> usize;

    fn max_reward(&self) -> f64;

    fn mean(&self, arm: usize) -> f64;

    fn reward(&self, arm: usize) -> f64;

    fn std(&self, arm: usize) -> f64;
}
