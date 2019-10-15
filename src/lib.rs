#[macro_use]
extern crate lazy_static;

use agent::Agent;
use bandit::Bandit;
pub use util::{Counter, RecordCounter};
use util::Stepper;

mod agent;
mod bandit;
mod game;
mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
