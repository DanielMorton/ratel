pub trait Stepper {
    fn reset(&mut self) -> ();

    fn step(&mut self, arm: usize) -> f64;
}
