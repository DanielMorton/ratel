use std::ops::AddAssign;

use num_traits::{Num, ToPrimitive};

use super::Counter;

/// Counter for cases where individual records must be maintained.
pub struct RecordCounter<T: ToPrimitive> {
    /// Vector of records passed to counter
    record: Vec<T>,
    /// Count of all elements passed to counter.
    counter: T,
}

impl<T: Num + ToPrimitive> RecordCounter<T> {
    /// Initializes counter to zero and record to empty
    pub fn new() -> RecordCounter<T> {
        RecordCounter {
            record: Vec::new(),
            counter: T::zero(),
        }
    }

    /// Returns the current vector of records.
    pub fn record(&self) -> &Vec<T> {
        &self.record
    }
}

impl<T: AddAssign + Num + ToPrimitive> Counter<T> for RecordCounter<T> {
    /// Returns the current value of the counter.
    fn counter(&self) -> &T {
        &self.counter
    }

    /// Resets counter to initial values.
    fn reset(&mut self) -> () {
        self.record = Vec::new();
        self.counter = T::zero()
    }

    /// Updates counter with new value.
    fn update(&mut self, n: T) -> () {
        self.record.push(n);
        self.counter += T::one()
    }
}

#[cfg(test)]
mod tests {
    use super::RecordCounter;
    use super::super::Counter;

    lazy_static! {
        static ref NUMS_VEC: Vec<i32> = vec![45, 5, 52, 93, 51, 90];
        static ref DEC_VEC: Vec<u32> = vec![79, 42, 11, 29, 76];
        static ref FLOAT_VEC: Vec<f64> = vec![9.62, 4.0, 4.6, 7.73, 7.59];
    }

    #[test]
    fn test_record_counter() {
        let mut ac: RecordCounter<i32> = RecordCounter::new();
        for i in 0..=5 {
            ac.update(NUMS_VEC[i])
        }
        assert_eq!(*ac.counter(), 6);
        assert_eq!(ac.record()[2], 52);
        ac.reset();

        assert_eq!(*ac.counter(), 0);
        for _ in 1..=7 {
            ac.update(3)
        }
        assert_eq!(*ac.counter(), 7);
    }

    #[test]
    fn test_record_int_counter() {
        let mut ac: RecordCounter<u32> = RecordCounter::new();
        for i in 0..=4 {
            ac.update(DEC_VEC[i])
        }
        assert_eq!(*ac.counter(), 5);
        assert_eq!(ac.record()[2], 11);
        ac.reset();
        assert_eq!(*ac.counter(), 0);
    }

    #[test]
    fn test_record_float_counter() {
        let mut ac: RecordCounter<f64> = RecordCounter::new();
        for i in 0..=4 {
            ac.update(FLOAT_VEC[i])
        }
        assert_eq!(*ac.counter(), 5.0);
        assert_eq!(ac.record()[2], 4.6);
        ac.reset();
        assert_eq!(*ac.counter(), 0.0);
    }
}
