pub use bandit::Bandit;
pub use binomial::BinomialBandit;
pub use gaussian::GaussianBandit;

use super::util::ArgBounds;

mod bandit;
mod binomial;
mod gaussian;
