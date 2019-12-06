//! This module contains the Agents that try to figure out the optimal strategy for playing a given Bandit.

pub use agent::Agent;
pub use epsilon_greedy::EpsilonGreedyAgent;
pub use greedy::GreedyAgent;
pub use optimistic::OptimisticAgent;

use super::util::ArgBounds;

mod agent;
mod epsilon_greedy;
mod greedy;
mod optimistic;
