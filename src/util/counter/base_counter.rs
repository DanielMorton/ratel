use std::ops::AddAssign;

use num_traits::Num;

use super::Counter;

struct BaseCounter<T: Num> {
    counter: T,
}

impl<T: Num + AddAssign> BaseCounter<T> {
    fn new() -> BaseCounter<T> {
        BaseCounter { counter: T::zero() }
    }

    fn increment(&mut self) -> () {
        self.counter += T::one();
    }
}

impl<T: Num + AddAssign> Counter<T> for BaseCounter<T> {
    fn counter(&self) -> &T {
        &self.counter
    }

    fn reset(&mut self) -> () {
        self.counter = T::zero()
    }

    fn update(&mut self, n: T) -> () {
        self.counter += n
    }
}
