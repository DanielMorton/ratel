
pub(super) trait Agent<T> {
    fn action(&self) -> usize;

    fn arms(&self) -> usize {
        self.q_star().len()
    }

    fn current_estimate(&self, arm: usize) -> f64 {
        self.q_star()[arm]
    }

    fn q_star(&self) -> &Vec<f64>;

    fn step(&mut self, arm: usize, reward: T) -> ();
}
