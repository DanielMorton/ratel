use std::ops::AddAssign;

use num_traits::{Num, ToPrimitive};

use super::Counter;

struct RecordCounter<T: ToPrimitive> {
    record: Vec<T>,
    counter: T,
}

impl<T: Num + ToPrimitive> RecordCounter<T> {
    fn new() -> RecordCounter<T> {
        RecordCounter {
            record: Vec::new(),
            counter: T::zero(),
        }
    }

    fn record(&self) -> &Vec<T> {
        &self.record
    }
}

impl<T: AddAssign + Num + ToPrimitive> Counter<T> for RecordCounter<T> {
    fn counter(&self) -> &T {
        &self.counter
    }

    fn reset(&mut self) -> () {
        self.record = Vec::new();
        self.counter = T::zero()
    }

    fn update(&mut self, n: T) -> () {
        self.record.push(n);
        self.counter += T::one()
    }
}
