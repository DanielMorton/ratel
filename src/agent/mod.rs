pub use agent::Agent;
pub use epsilon_greedy::EpsilonGreedyAgent;
pub use greedy::GreedyAgent;

use super::util::ArgBounds;

mod agent;
mod epsilon_greedy;
mod greedy;
mod optimistic;
