//! This module contains the Bandits whose behavior the Agents study.

pub use bandit::Bandit;
pub use binomial::BinomialBandit;
pub use exponential::ExponentialBandit;
pub use gaussian::GaussianBandit;
pub use log_normal::LogNormalBandit;

use super::util::ArgBounds;

mod bandit;
mod binomial;
mod exponential;
mod gamma;
mod gaussian;
mod log_normal;
