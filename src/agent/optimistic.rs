use num_traits::ToPrimitive;

use crate::Stepper;

use super::{Agent, ArgBounds};

pub struct OptimisticAgent<'a> {
    q_star: Vec<f64>,
    c: f64,
    total: f64,
    arm_total: Vec<f64>,
    stepper: &'a mut dyn Stepper,
}

impl<'a, T: ToPrimitive> Agent<T> for OptimisticAgent<'a> {
    fn action(&self) -> usize {
        self.q_star
            .iter()
            .zip(&self.arm_total)
            .map(|(q, c)| *q + self.c * (self.total.ln() / *c).sqrt())
            .collect::<Vec<f64>>()
            .arg_max()
    }

    fn q_star(&self) -> &Vec<f64> {
        &self.q_star
    }

    fn reset(&mut self, q_init: Vec<f64>) {
        self.q_star = q_init;
        self.stepper.reset()
    }

    fn step(&mut self, arm: usize, reward: T) -> () {
        self.q_star[arm] += self.update(arm, reward);
        self.arm_total[arm] += 1.0;
        self.total += 1.0
    }

    fn stepper(&mut self) -> &mut dyn Stepper {
        self.stepper
    }
}

impl<'a> OptimisticAgent<'a> {
    pub fn new(q_init: Vec<f64>, c: f64, stepper: &mut dyn Stepper) -> OptimisticAgent {
        OptimisticAgent {
            arm_total: vec![0.0; q_init.len()],
            q_star: q_init,
            c,
            total: 0.0,
            stepper,
        }
    }

    fn arm_total(&self) -> &Vec<f64> {
        &self.arm_total
    }
}
