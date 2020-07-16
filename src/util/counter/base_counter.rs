use std::ops::AddAssign;

use num_traits::Num;

use super::Counter;

/// Counter for simple increments.
struct BaseCounter<T: Num> {
    /// Current value of counter.
    counter: T,
}

impl<T: Num + AddAssign> BaseCounter<T> {
    /// Initializes a new base counter with initial value 0.
    fn new() -> BaseCounter<T> {
        BaseCounter { counter: T::zero() }
    }

    /// Increments the base counter by 1.
    fn increment(&mut self) -> () {
        self.counter += T::one();
    }
}

impl<T: Num + AddAssign> Counter<T> for BaseCounter<T> {
    /// Returns the current value of the counter.
    fn counter(&self) -> &T {
        &self.counter
    }

    /// Resets the counter to its initial value.
    fn reset(&mut self) -> () {
        self.counter = T::zero()
    }

    /// Updates the counter by some value.
    fn update(&mut self, n: T) -> () {
        self.counter += n
    }
}

#[cfg(test)]
mod tests {
    use super::BaseCounter;
    use super::super::Counter;

    lazy_static! {
        static ref POS_VEC: Vec<u32> = vec![3, 7, 9, 8, 4, 10, 5];
        static ref NUMS_VEC: Vec<i32> = vec![10, -8, 0, -4, 8, 9, -7];
        static ref FLOAT_VEC: Vec<f64> = vec![8.44, 4.93, -3.67, 8.91, -4.74, -1.67, 6.26];
    }

    #[test]
    fn test_base_counter() {
        let mut bc: BaseCounter<u32> = BaseCounter::new();
        for _ in 1..=6 {
            bc.increment()
        }
        assert_eq!(*bc.counter(), 6);
        bc.reset();
        assert_eq!(*bc.counter(), 0);
        for i in 0..=6 {
            bc.update(POS_VEC[i])
        }
        assert_eq!(*bc.counter(), 46);
        bc.reset();
        assert_eq!(*bc.counter(), 0)
    }

    #[test]
    fn test_base_int_counter() {
        let mut bc: BaseCounter<i32> = BaseCounter::new();
        for i in 0..=6 {
            bc.update(NUMS_VEC[i])
        }
        assert_eq!(*bc.counter(), 8);
        bc.reset();
        assert_eq!(*bc.counter(), 0)
    }

    #[test]
    fn test_base_float_counter() {
        let mut bc: BaseCounter<f64> = BaseCounter::new();
        for i in 0..=6 {
            bc.update(FLOAT_VEC[i])
        }
        assert_eq!(*bc.counter(), 18.46);
        bc.reset();
        assert_eq!(*bc.counter(), 0.0)
    }
}
