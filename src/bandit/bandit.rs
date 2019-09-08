use super::util::arg_max;

pub(super) struct Bandit {
    q_values: Vec<f64>,
}

impl Bandit {
    fn arms(&self) -> usize {
        self.q_values.len()
    }

    fn best_arm(&self) -> usize {
        arg_max(&self.q_values)
    }

    fn max_reward(&self) -> f64 {
        *self
            .q_values
            .iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::Bandit;

    lazy_static! {
        static ref BANDIT: Bandit = Bandit {
            q_values: vec![0.46, 0.61, 0.19, 0.5, 0.99]
        };
    }

    #[test]
    fn test_arms() {
        assert_eq!(BANDIT.arms(), 5)
    }

    #[test]
    fn test_best_arm() {
        assert_eq!(BANDIT.best_arm(), 4)
    }

    #[test]
    fn test_max_reward() {
        assert_eq!(BANDIT.max_reward(), 0.99)
    }
}
