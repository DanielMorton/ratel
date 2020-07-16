use std::ops::AddAssign;

use num_traits::{Num, ToPrimitive};

use super::Counter;

/// A counter to record aggregate totals and averages.
struct AggregateCounter<T: ToPrimitive> {
    /// Sum of all elements passed to counter.
    total: T,
    /// Count of all elements passed to counter.
    counter: T,
}

impl<T: Num + ToPrimitive> AggregateCounter<T> {
    /// Computes average of values passed to counter.
    fn average(&self) -> f64 {
        self.total.to_f64().unwrap() / self.counter.to_f64().unwrap()
    }

    /// Initializes counter to zero.
    fn new() -> AggregateCounter<T> {
        AggregateCounter {
            total: T::zero(),
            counter: T::zero(),
        }
    }

    /// Returns sum of all values passed to counter.
    fn total(&self) -> &T {
        &self.total
    }
}

impl<T: AddAssign + Num + ToPrimitive> Counter<T> for AggregateCounter<T> {
    /// Returns the current value of the counter.
    fn counter(&self) -> &T {
        &self.counter
    }

    /// Resets the counter to its initial value.
    fn reset(&mut self) -> () {
        self.total = T::zero();
        self.counter = T::zero()
    }

    /// Updates the counter by some value.
    fn update(&mut self, n: T) -> () {
        self.total += n;
        self.counter += T::one()
    }
}

#[cfg(test)]
mod tests {
    use super::AggregateCounter;
    use super::super::Counter;

    lazy_static! {
        static ref NUMS_VEC: Vec<i32> = vec![45, 5, 52, 93, 51, 90];
        static ref DEC_VEC: Vec<u32> = vec![79, 42, 11, 29, 76];
        static ref FLOAT_VEC: Vec<f64> = vec![9.62, 4.0, 4.6, 7.73, 7.59];
    }

    #[test]
    fn test_aggregate_counter() {
        let mut ac: AggregateCounter<i32> = AggregateCounter::new();
        for i in 0..=5 {
            ac.update(NUMS_VEC[i])
        }
        assert_eq!(*ac.counter(), 6);
        assert_eq!(*ac.total(), 336);
        assert_eq!(ac.average(), 56.0);
        ac.reset();

        assert_eq!(*ac.counter(), 0);
        for _ in 1..=7 {
            ac.update(3)
        }
        assert_eq!(*ac.counter(), 7);
        assert_eq!(*ac.total(), 21);
        assert_eq!(ac.average(), 3.0);
    }

    #[test]
    fn test_aggregate_int_counter() {
        let mut ac: AggregateCounter<u32> = AggregateCounter::new();
        for i in 0..=4 {
            ac.update(DEC_VEC[i])
        }
        assert_eq!(*ac.counter(), 5);
        assert_eq!(*ac.total(), 237);
        assert_eq!(ac.average(), 47.4);
        ac.reset();
        assert_eq!(*ac.counter(), 0);
    }

    #[test]
    fn test_aggregate_float_counter() {
        let mut ac: AggregateCounter<f64> = AggregateCounter::new();
        for i in 0..=4 {
            ac.update(FLOAT_VEC[i])
        }
        assert_eq!(*ac.counter(), 5.0);
        assert_eq!(*ac.total(), 33.54);
        assert_eq!(ac.average(), 6.708);
        ac.reset();
        assert_eq!(*ac.counter(), 0.0);
    }
}
