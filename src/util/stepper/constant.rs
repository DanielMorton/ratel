use super::Stepper;

pub struct ConstantStepper {
    step_size: f64,
}

impl ConstantStepper {
    fn new(step_size: f64) -> ConstantStepper {
        assert!(step_size > 0.0);
        ConstantStepper { step_size }
    }
}

impl Stepper for ConstantStepper {
    fn reset(&mut self) -> () {}

    fn step(&mut self, arm: usize) -> f64 {
        self.step_size
    }
}
