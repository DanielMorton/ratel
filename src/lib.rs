#[macro_use]
extern crate lazy_static;

pub use agent::{Agent, EpsilonGreedyAgent, GreedyAgent, OptimisticAgent};
pub use bandit::{Bandit, BinomialBandit, GaussianBandit};
pub use game::Game;
pub use run::{
    epsilon_bernoulli, greedy_bernoulli, pool_bernoulli,
    sequential_bernoulli,
};
pub use util::{Counter, HarmonicStepper, print_hms, RecordCounter, Stepper};

mod agent;
mod bandit;
mod game;
mod run;
mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
