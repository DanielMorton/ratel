//! This module contains the Bandits whose behavior the Agents study.

pub use bandit::Bandit;
pub use binomial::BinomialBandit;
pub use gaussian::GaussianBandit;

use super::util::ArgBounds;

mod bandit;
mod binomial;
mod gaussian;
