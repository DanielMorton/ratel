use std::fs::File;
use std::io::Write;

use clap::{ArgMatches, value_t};
use rand_distr::uniform::Uniform;

use super::{
    Agent, BinomialBandit, EpsilonGreedyAgent, Game, GreedyAgent, HarmonicStepper, multiple_runs,
    random_init,
};

pub fn sequential_bernoulli(runs: u32, iterations: u32, agent_start: f64, arg: &ArgMatches) {
    let rand_start = Uniform::new(agent_start - 1e-7, agent_start + 1e-7);
    let reward_vec: Vec<f64> = (1..=9).into_iter().map(|x| f64::from(x) / 10.0).collect();
    let increment_vec: Vec<f64> = vec![0.04, 0.09, 0.02, 0.06, 0.07, 0.08, 0.03, 0.01, 0.05, 0.0];
    let mut q_init = random_init(&rand_start, increment_vec.len());
    let mut stepper = HarmonicStepper::new(1, increment_vec.len());
    let ones = vec![1; increment_vec.len()];
    reward_vec.into_iter().for_each(|x| {
        let rewards = (&increment_vec).into_iter().map(|&i| i + x).collect();
        let (mut agent, file_name): (Box<dyn Agent<u32>>, String) = if arg.is_present("pair_greedy")
        {
            (
                Box::new(GreedyAgent::new(q_init.clone(), &mut stepper)),
                format!("results/greedy/greedy_{}_{}.csv", x, agent_start),
            )
        } else if arg.is_present("pair_epsilon") {
            let epsilon = value_t!(arg.value_of("pair_epsilon"), f64).unwrap_or_else(|e| e.exit());
            (
                Box::new(EpsilonGreedyAgent::new(
                    q_init.clone(),
                    &mut stepper,
                    epsilon,
                )),
                format!(
                    "results/epsilon/epsilon_e{}_{}_{}.csv",
                    epsilon, x, agent_start
                ),
            )
        } else {
            (
                Box::new(GreedyAgent::new(q_init.clone(), &mut stepper)),
                format!("bad_file.csv"),
            )
        };
        let bandit = BinomialBandit::new(&ones, &rewards);
        let mut game = Game::new(&mut *agent, &bandit);
        multiple_runs(&mut game, runs, iterations, &rand_start, file_name)
    })
}
