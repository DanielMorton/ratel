use std::ops::AddAssign;

use num_traits::{Num, ToPrimitive};

use super::Counter;

struct AggregateCounter<T: ToPrimitive> {
    total: T,
    counter: T,
}

impl<T: Num + ToPrimitive> AggregateCounter<T> {
    fn average(&self) -> f64 {
        self.total.to_f64().unwrap() / self.counter.to_f64().unwrap()
    }

    fn new() -> AggregateCounter<T> {
        AggregateCounter {
            total: T::zero(),
            counter: T::zero(),
        }
    }

    fn total(&self) -> &T {
        &self.total
    }
}

impl<T: Num + ToPrimitive + AddAssign> Counter<T> for AggregateCounter<T> {
    fn counter(&self) -> &T {
        &self.counter
    }

    fn reset(&mut self) -> () {
        self.total = T::zero();
        self.counter = T::zero()
    }

    fn update(&mut self, n: T) -> () {
        self.total += n;
        self.counter += T::one()
    }
}
