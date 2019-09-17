use rand::distributions::{DistIter, Distribution};
use rand::Rng;

pub(super) struct Distributions<D, R>
    where
        D: Distribution<f64>,
        R: Rng,
{
    distributions: Vec<DistIter<D, R, f64>>,
}

impl<D, R> Distributions<D, R>
    where
        D: Distribution<f64>,
        R: Rng,
{
    pub fn arms(&self) -> usize {
        self.distributions.len()
    }

    pub fn reward(&mut self, arm: usize) -> f64 {
        self.distributions[arm].next().unwrap()
    }
}
