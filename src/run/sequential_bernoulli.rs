use std::fs::File;
use std::io::Write;

use super::{epsilon_bernoulli, greedy_bernoulli};

pub fn sequential_bernoulli(runs: u32, iterations: u32, agent_start: f64, epsilon: f64) {
    let reward_vec: Vec<f64> = (1..=9).into_iter().map(|x| f64::from(x) / 10.0).collect();
    let increment_vec: Vec<f64> = vec![0.04, 0.09, 0.02, 0.06, 0.07, 0.08, 0.03, 0.01, 0.05, 0.0];
    reward_vec.into_iter().for_each(|x| {
        let rewards = (&increment_vec).into_iter().map(|i| i + x).collect();
        let (wins, rewards) = if epsilon == 0.0 {
            greedy_bernoulli(runs, iterations, agent_start, rewards)
        } else {
            epsilon_bernoulli(runs, iterations, agent_start, epsilon, rewards)
        };
        let ge = if epsilon == 0.0 { "greedy" } else { "epsilon" };
        let mut file =
            File::create(format!("{ge}/{ge}_{}_{}.csv", x, agent_start, ge = ge)).unwrap();
        let output = wins
            .into_iter()
            .zip(rewards.into_iter())
            .map(|x| format!("{}, {}", x.0, x.1))
            .fold(String::from("wins,rewards"), |s, x| [s, x].join("\n"));

        file.write_all(output.as_bytes()).unwrap();
    })
}
