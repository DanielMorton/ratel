#[macro_use]
extern crate lazy_static;

use util::Stepper;

mod agent;
mod bandit;
mod util;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
