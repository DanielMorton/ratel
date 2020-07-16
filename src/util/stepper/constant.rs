use super::Stepper;

/// Stepper using a constant value.
pub struct ConstantStepper {
    /// Step multiple for update rule.
    step_size: f64,
}

impl ConstantStepper {
    /// Initialize stepper with positive value.
    fn new(step_size: f64) -> ConstantStepper {
        assert!(step_size > 0.0);
        ConstantStepper { step_size }
    }

    /// Updates the step size.
    fn update(&mut self, step_size: f64) {
        self.step_size = step_size
    }
}

impl Stepper for ConstantStepper {
    /// Resets the stepper. Has no effect on ConstantStepper.
    fn reset(&mut self) {}

    /// Return the current step size. Used for apply update rule.
    fn step(&mut self, _arm: usize) -> f64 {
        self.step_size
    }
}
