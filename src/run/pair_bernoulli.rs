use std::fs::File;
use std::io::Write;

use clap::{ArgMatches, value_t};
use itertools::Itertools;
use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::uniform::Uniform;
use scoped_threadpool::Pool;

use super::{Agent, BinomialBandit, EpsilonGreedyAgent, Game, GreedyAgent,
            HarmonicStepper, OptimisticAgent,
};

pub fn pool_bernoulli(runs: u32, iterations: u32, agent_start: f64, arg: &ArgMatches) {
    let reward_vec: Vec<f64> = (1..=99).into_iter().map(|x| f64::from(x) / 100.0).collect();
    let rand_start = Uniform::new(agent_start - 1e-7, agent_start + 1e-7);
    let pair_vec: Vec<Vec<f64>> = reward_vec
        .iter()
        .cartesian_product(&reward_vec)
        .filter(|(&x, &y)| x < y)
        .map(|(&x, &y)| vec![x, y])
        .collect();
    let mut pool = Pool::new(12);
    pool.scoped(|scope| {
        pair_vec.into_iter().for_each(|pair| {
            scope.execute(move || {
                pair_bernoulli(runs, iterations, agent_start, pair, &rand_start, arg);
            })
        })
    })
}

fn random_init(rand_start: &Uniform<f64>, len: usize) -> Vec<f64> {
    (1..=len)
        .into_iter()
        .map(|_| rand_start.sample(&mut thread_rng()))
        .collect()
}

pub fn pair_bernoulli(runs: u32, iterations: u32, agent_start: f64, pair: Vec<f64>, rand_start: &Uniform<f64>, arg: &ArgMatches) {
    let q_init = random_init(rand_start, pair.len());
    let mut stepper = HarmonicStepper::new(1, pair.len());
    let (mut agent, file_name): (Box<dyn Agent<u32>>, String) = if arg.is_present("pair_greedy") {
        (Box::new(GreedyAgent::new(q_init, &mut stepper)), format!(
            "results/pair/pair_a{}_{}_{}.csv",
            agent_start, pair[0], pair[1]
        ))
    } else if arg.is_present("pair_epsilon") {
        let epsilon = value_t!(arg.value_of("pair_epsilon"), f64).unwrap_or_else(|e| e.exit());
        (Box::new(EpsilonGreedyAgent::new(q_init, &mut stepper, epsilon)), format!(
            "results/pair/epsilon_e{}_a{}_{}_{}.csv",
            epsilon, agent_start, pair[0], pair[1]
        ))
    } else if arg.is_present("pair_optimistic") {
        let c = value_t!(arg.value_of("pair_optimistic"), f64).unwrap_or_else(|e| e.exit());
        (Box::new(OptimisticAgent::new(q_init, c, &mut stepper)), format!(
            "results/pair/optimistic_c{}_a{}_{}_{}.csv",
            c, agent_start, pair[0], pair[1]
        ))
    } else {
        (Box::new(GreedyAgent::new(q_init, &mut stepper)), format!("bad_file.csv"))
    };

    let ones = vec![1; pair.len()];
    let bandit = BinomialBandit::new(&ones, &pair);
    let mut game = Game::new(&mut *agent, &bandit);
    let mut wins = vec![0u32; iterations as usize];
    let mut rewards = vec![0u32; iterations as usize];

    for _ in 0..runs {
        game.run(iterations);
        wins = wins
            .into_iter()
            .zip(game.wins().into_iter())
            .map(|(w, &gw)| w + gw)
            .collect();
        rewards = rewards
            .into_iter()
            .zip(game.rewards().into_iter())
            .map(|(r, &gr)| r + gr)
            .collect();
        game.reset(random_init(rand_start, pair.len()))
    }
    let greedy = wins.iter()
        .map(|&w| f64::from(w) / f64::from(runs))
        .zip(rewards.iter()
            .map(|&r| f64::from(r) / f64::from(runs)))
        .map(|(w, r)| format!("{}, {}", w, r))
        .fold(String::from("wins,rewards"), |s, s0| [s, s0].join("\n"));
    let mut file = File::create(file_name).unwrap();
    file.write_all(greedy.as_bytes()).unwrap();
}
