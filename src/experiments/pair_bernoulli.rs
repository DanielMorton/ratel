use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

use clap::{ArgMatches, value_t};
use itertools::Itertools;
use rand_distr::uniform::Uniform;
use scoped_threadpool::Pool;

use super::{
    Agent, BinomialBandit, EpsilonGreedyAgent, Game, GreedyAgent, HarmonicStepper, multiple_runs,
    OptimisticAgent, random_init,
};

fn make_file_name(arg: &ArgMatches) -> String {
    if arg.is_present("pair_greedy") {
        String::from("results2/pair/greedy.csv")
    } else if arg.is_present("pair_epsilon") {
        let epsilon = value_t!(arg.value_of("pair_epsilon"), f64).unwrap_or_else(|e| e.exit());
        format!("results2/pair/epsilon_e{}.csv", epsilon)
    } else if arg.is_present("pair_optimistic") {
        let c = value_t!(arg.value_of("pair_optimistic"), f64).unwrap_or_else(|e| e.exit());
        format!("results2/pair/optimistic_c{}.csv", c)
    } else {
        String::from("bad_file.csv")
    }
}

pub fn pool_bernoulli(runs: u32, iterations: u32, agent_start: &[f64], arg: &ArgMatches) {
    let file_name = make_file_name(arg);
    let mut file = File::create(&file_name).unwrap();
    let first_line = "left,right,start,iteration,wins,rewards";
    file.write_all(first_line.as_bytes()).unwrap();

    let reward_vec: Vec<f64> = (1..=99).map(|x| f64::from(x) / 100.0).collect();
    //let rand_start = Uniform::new(agent_start - 1e-7, agent_start + 1e-7);

    let pair_vec = reward_vec
        .iter()
        .cartesian_product(&reward_vec)
        .filter(|(&x, &y)| x < y)
        .map(|(&x, &y)| (x, y))
        .collect::<Vec<(f64, f64)>>();
    let agent_vec = pair_vec
        .iter()
        .cartesian_product(agent_start)
        .map(|(&(x, y), &s)| (x, y, s))
        .collect::<Vec<(f64, f64, f64)>>();
    let mut pool = Pool::new(12);
    pool.scoped(|scope| {
        agent_vec.iter().for_each(|(left, right, start)| {
            scope.execute(move || {
                pair_bernoulli(runs, iterations, left, right, start, arg);
            })
        })
    })
}

fn pair_bernoulli(
    runs: u32,
    iterations: u32,
    left: &f64,
    right: &f64,
    start: &f64,
    arg: &ArgMatches,
) {
    let rand_start = Uniform::new(start - 1e-7, start + 1e-7);
    let q_init = random_init(&rand_start, 2);
    let mut stepper = HarmonicStepper::new(1, 2);
    let ones = vec![1; 2];
    let pair = vec![*left, *right];
    let bandit = BinomialBandit::new(&ones, &pair);
    let mut agent: Box<dyn Agent<u32>> = if arg.is_present("pair_greedy") {
        Box::new(GreedyAgent::new(q_init, &mut stepper))
    } else if arg.is_present("pair_epsilon") {
        let epsilon = value_t!(arg.value_of("pair_epsilon"), f64).unwrap_or_else(|e| e.exit());
        Box::new(EpsilonGreedyAgent::new(q_init, &mut stepper, epsilon))
    } else if arg.is_present("pair_optimistic") {
        let c = value_t!(arg.value_of("pair_optimistic"), f64).unwrap_or_else(|e| e.exit());
        Box::new(OptimisticAgent::new(q_init, c, &mut stepper))
    } else {
        Box::new(GreedyAgent::new(q_init, &mut stepper))
    };

    let mut game = Game::new(&mut *agent, &bandit);

    let file_name = make_file_name(arg);
    let mut file = OpenOptions::new().append(true).open(file_name).unwrap();
    multiple_runs(
        &mut game,
        &pair,
        start,
        runs,
        iterations,
        &rand_start,
        &mut file,
    )
}
