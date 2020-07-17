//! # Ratel
//!
//! A library for running multi-armed bandit simulations.
//!
//! The simulation has two components: a Bandit with some number of arms which each has a payout
//! according to some distribution, and an Agent that uses some strategy to learn the optimal
//! bandit arm. A Game then consists of an Agent learning about a Bandit by pulling arms according
//! to its preferred strategy.

#[macro_use]
extern crate lazy_static;

pub use agent::{Agent, EpsilonGreedyAgent, GreedyAgent, OptimisticAgent};
pub use bandit::{Bandit, BinomialBandit, GaussianBandit};
pub use experiments::pool_bernoulli;
pub use game::Game;
pub use util::{Counter, HarmonicStepper, print_hms, RecordCounter, Stepper};

mod agent;
mod bandit;
mod experiments;
mod game;
mod util;
