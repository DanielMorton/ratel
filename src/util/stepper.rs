pub trait Stepper {
    fn reset(&mut self) -> ();

    fn step(&mut self, arm: usize) -> f64;
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

    fn step(&mut self, arm: usize) -> f64 {
        self.step_size
    }
}

pub struct HarmonicStepper {
    warm_up: u32,
    step_size: Vec<u32>,
}

impl HarmonicStepper {
    pub fn new(step_size: u32, length: usize) -> HarmonicStepper {
        assert!(step_size > 0);
        assert!(length > 0);
        HarmonicStepper {
            warm_up: step_size,
            step_size: vec![step_size; length],
        }
    }

    pub fn arms(&self) -> usize {
        self.step_size.len()
    }
}

impl Stepper for HarmonicStepper {
    fn reset(&mut self) -> () {
        self.step_size = vec![self.warm_up; self.step_size.len()]
    }

    fn step(&mut self, arm: usize) -> f64 {
        let s = 1.0 / f64::from(self.step_size[arm]);
        self.step_size[arm] += 1;
        s
    }
}
