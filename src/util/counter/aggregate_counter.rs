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

impl<T: AddAssign + Num + ToPrimitive> Counter<T> for AggregateCounter<T> {
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

#[cfg(test)]
mod tests {
    use super::AggregateCounter;
    use super::super::Counter;

    #[test]
    fn test_aggregate_counter() {
        let mut ac: AggregateCounter<u32> = AggregateCounter::new();
        for i in 1..=6 {
            ac.update(i)
        }
        assert_eq!(*ac.counter(), 6);
        assert_eq!(*ac.total(), 21);
        assert_eq!(ac.average(), 3.5);
        ac.reset();
        assert_eq!(*ac.counter(), 0);
        for _ in 1..=7 {
            ac.update(3)
        }
        assert_eq!(*ac.counter(), 7);
        assert_eq!(*ac.total(), 21);
        assert_eq!(ac.average(), 3.0);
    }
}
