use super::util::arg_max;

pub struct Bandit {
    q_values: Vec<f64>
}

impl Bandit {

    fn arms(&self) -> usize {
        self.q_values.len()
    }

    fn best_arm(&self) -> usize {
        arg_max(&self.q_values)
    }
}