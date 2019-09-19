pub(in super::super) trait Stepper {
    fn reset(&mut self) -> ();

    fn step(&mut self) -> f64;
}

struct ConstantStepper {
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

    fn step(&mut self) -> f64 {
        self.step_size
    }
}

struct HarmonicStepper {
    warm_up: u32,
    step_size: u32,
}

impl HarmonicStepper {
    fn new(step_size: u32) -> HarmonicStepper {
        assert!(step_size > 0);
        HarmonicStepper {
            warm_up: step_size,
            step_size,
        }
    }
}

impl Stepper for HarmonicStepper {
    fn reset(&mut self) -> () {
        self.step_size = self.warm_up
    }

    fn step(&mut self) -> f64 {
        let s = 1.0 / (self.step_size as f64);
        self.step_size += 1;
        s
    }
}
