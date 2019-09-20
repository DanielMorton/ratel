use std::iter::Sum;
use std::ops::AddAssign;

use num_traits::Num;

pub(in super::super) trait Counter<T> {
    fn counter(&self) -> u32;

    fn reset(&mut self) -> ();

    fn update(&mut self, n: T) -> ();
}

struct AggregateCounter {
    counter: u32
}

impl AggregateCounter {
    fn new() -> AggregateCounter {
        AggregateCounter { counter: 0 }
    }

    fn increment(&mut self) -> () {
        self.counter += 1;
    }
}

impl Counter<u32> for AggregateCounter {
    fn counter(&self) -> u32 {
        self.counter
    }

    fn reset(&mut self) -> () {
        self.counter = 0
    }

    fn update(&mut self, n: u32) -> () {
        self.counter += n
    }
}

struct AverageCounter<T: Num + AddAssign + Copy + Sum> {
    total: T,
    counter: u32,
}

impl<T: Num + AddAssign + Copy + Sum> AverageCounter<T> {
    fn average(&self) -> T {
        self.total() / vec![T::one(); self.counter as usize].iter().copied().sum()
    }


    fn new() -> AverageCounter<T> {
        AverageCounter { total: T::zero(), counter: 0 }
    }

    fn total(&self) -> T {
        self.total
    }
}

impl<T: Num + AddAssign + Copy + Sum> Counter<T> for AverageCounter<T> {
    fn counter(&self) -> u32 {
        self.counter
    }

    fn reset(&mut self) -> () {
        self.total = T::zero();
        self.counter = 0
    }

    fn update(&mut self, n: T) -> () {
        self.total += n;
        self.counter += 1
    }
}