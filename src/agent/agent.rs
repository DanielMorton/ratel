

pub(super) trait Agent<T> {
    fn action(&self) -> usize;

    fn arms(&self) -> usize;

    fn current_estimate(&self, arm: usize) -> f64;

    fn step(&mut self, arm: usize, reward: T) -> ();
}
