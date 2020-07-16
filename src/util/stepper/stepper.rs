/// Trait containing all methods required by all steppers.
pub trait Stepper {
    /// Resets the stepper to its initial value.
    fn reset(&mut self) -> ();

    /// Returns the current step size.
    fn step(&mut self, arm: usize) -> f64;
}
