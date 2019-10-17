pub use epsilon_bernoulli::epsilon_bernoulli;
pub use greedy_bernoulli::greedy_bernoulli;
pub use sequential_bernoulli::sequential_bernoulli;

use super::{BinomialBandit, EpsilonGreedyAgent, Game, GreedyAgent, HarmonicStepper};

mod epsilon_bernoulli;
mod greedy_bernoulli;
mod sequential_bernoulli;
