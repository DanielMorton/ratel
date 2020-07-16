use num_traits::Num;

/// A trait for common methods required by all counters.
pub trait Counter<T: Num> {
    /// Returns the current value of the counter.
    fn counter(&self) -> &T;

    /// Resets the counter to its initial values.
    fn reset(&mut self) -> ();

    /// Updates the counter with a new value.
    fn update(&mut self, n: T) -> ();
}
