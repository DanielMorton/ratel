use std::marker::PhantomData;

use num_traits::ToPrimitive;

use crate::Stepper;

use super::{Agent, ArgBounds};

pub struct OptimisticAgent<'a, T> {
    q_star: Vec<f64>,
    c: f64,
    total: f64,
    arm_total: Vec<f64>,
    stepper: &'a mut dyn Stepper,
    phantom: PhantomData<T>,
}

impl<'a, T: ToPrimitive> Agent<T> for OptimisticAgent<'a, T> {
    fn action(&self) -> usize {
        self.q_star
            .iter()
            .zip(&self.arm_total)
            .map(|(&q, &c)| q + self.c * (self.total.ln() / c).sqrt())
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

impl<'a, T> OptimisticAgent<'a, T> {
    pub fn new(q_init: Vec<f64>, c: f64, stepper: &'a mut dyn Stepper) -> OptimisticAgent<'a, T> {
        assert!(c > 0.0);
        OptimisticAgent {
            arm_total: vec![1.0; q_init.len()],
            q_star: q_init,
            c,
            total: 1.0,
            stepper,
            phantom: PhantomData,
        }
    }

    fn arm_total(&self) -> &Vec<f64> {
        &self.arm_total
    }
}

#[cfg(test)]
mod tests {
    use crate::{HarmonicStepper, Stepper};

    use super::{Agent, OptimisticAgent};

    lazy_static! {
        static ref Q_INIT: Vec<f64> = vec![0.5, 0.61, 0.7, 0.12, 0.37];
    }

    #[test]
    fn test_new() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let c = 2.0;
        let optimistic: OptimisticAgent<u32> =
            OptimisticAgent::new(Q_INIT.to_vec(), c, &mut stepper);
        assert_eq!(optimistic.c, c);
        assert_eq!(optimistic.q_star, vec![0.5, 0.61, 0.7, 0.12, 0.37])
    }

    #[test]
    fn test_action() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let c = 2.0;
        let optimistic: OptimisticAgent<u32> =
            OptimisticAgent::new(Q_INIT.to_vec(), c, &mut stepper);
        assert_eq!(optimistic.action(), 2)
    }

    #[test]
    fn test_q_star() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let c = 2.0;
        let optimistic: OptimisticAgent<u32> =
            OptimisticAgent::new(Q_INIT.to_vec(), c, &mut stepper);
        assert_eq!(optimistic.q_star(), &vec![0.5, 0.61, 0.7, 0.12, 0.37])
    }

    #[test]
    fn test_reset() {
        let mut stepper = HarmonicStepper::new(1, Q_INIT.len());
        let c = 2.0;
        let mut optimistic: OptimisticAgent<u32> =
            OptimisticAgent::new(Q_INIT.to_vec(), c, &mut stepper);
        let new_q = vec![0.01, 0.86, 0.43, 0.65, 0.66];
        optimistic.reset(new_q.clone());
        assert_eq!(optimistic.q_star(), &new_q)
    }
}
