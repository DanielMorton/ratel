use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::uniform::Uniform;

pub use pair_bernoulli::pool_bernoulli;
pub use sequential_bernoulli::sequential_bernoulli;

use super::{
    Agent, BinomialBandit, EpsilonGreedyAgent, Game, GreedyAgent, HarmonicStepper, OptimisticAgent,
};

mod pair_bernoulli;
mod sequential_bernoulli;

fn random_init(rand_start: &Uniform<f64>, len: usize) -> Vec<f64> {
    (1..=len)
        .into_iter()
        .map(|_| rand_start.sample(&mut thread_rng()))
        .collect()
}
