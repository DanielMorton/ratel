use super::Stepper;

/// Stepper with a harmonically decreasing step size.
///
/// Stores step sizes as integers and returns their inverses when called.
pub struct HarmonicStepper {
    /// Initial value of step size.
    warm_up: u32,
    /// Vector of (inverse) step sizes for each bandit arm.
    step_size: Vec<u32>,
}

impl HarmonicStepper {
    /// Initializes HarmonicStepper with the same step size for each bandit arm.
    pub fn new(step_size: u32, length: usize) -> HarmonicStepper {
        assert!(step_size > 0);
        assert!(length > 0);
        HarmonicStepper {
            warm_up: step_size,
            step_size: vec![step_size; length],
        }
    }

    /// Returns the number of arms in the bandit.
    pub fn arms(&self) -> usize {
        self.step_size.len()
    }
}

impl Stepper for HarmonicStepper {
    /// Resets all step sizes to initial value.
    fn reset(&mut self) {
        self.step_size = vec![self.warm_up; self.step_size.len()]
    }

    /// Returns current step size for the given arm and increments step size for that arm.
    fn step(&mut self, arm: usize) -> f64 {
        let s = 1.0 / f64::from(self.step_size[arm]);
        self.step_size[arm] += 1;
        s
    }
}
