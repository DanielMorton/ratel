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

#[cfg(test)]
mod tests {
    use super::BaseCounter;
    use super::super::Counter;

    #[test]
    fn test_base_counter() {
        let mut bc: BaseCounter<u32> = BaseCounter::new();
        for _ in 1..=6 {
            bc.increment()
        }
        assert_eq!(*bc.counter(), 6);
        bc.reset();
        assert_eq!(*bc.counter(), 0);
        for i in 1..=6 {
            bc.update(i)
        }
        assert_eq!(*bc.counter(), 21);
        bc.reset();
        assert_eq!(*bc.counter(), 0)
    }
}
