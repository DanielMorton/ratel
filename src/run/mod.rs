pub use epsilon_bernoulli::epsilon_bernoulli;
pub use greedy_bernoulli::greedy_bernoulli;
pub use optimistic_bernoulli::optimistic_bernoulli;
pub use pair_bernoulli::{pair_epsilon, pair_greedy, pair_optimistic};
pub use sequential_bernoulli::sequential_bernoulli;

use super::{BinomialBandit, EpsilonGreedyAgent, Game, GreedyAgent, HarmonicStepper, OptimisticAgent};

mod epsilon_bernoulli;
mod greedy_bernoulli;
mod optimistic_bernoulli;
mod pair_bernoulli;
mod sequential_bernoulli;
