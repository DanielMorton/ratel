use std::fs::File;
use std::io::Write;

use itertools::Itertools;
use scoped_threadpool::Pool;

use super::{epsilon_bernoulli, greedy_bernoulli, optimistic_bernoulli};

pub fn pair_greedy(runs: u32, iterations: u32, agent_start: f64) {
    let reward_vec: Vec<f64> = (1..=99).into_iter().map(|x| f64::from(x) / 100.0).collect();
    let pair_vec: Vec<Vec<f64>> = (&reward_vec)
        .into_iter()
        .cartesian_product((&reward_vec).into_iter())
        .filter(|(x, y)| x < y)
        .map(|(x, y)| vec![*x, *y])
        .collect();
    let mut pool = Pool::new(12);
    pool.scoped(|scope| {
        pair_vec.into_iter().for_each(|pair| {
            scope.execute(move || {
                let mut file = File::create(format!(
                    "results/pair/pair_a{}_{}_{}.csv",
                    agent_start, pair[0], pair[1]
                ))
                    .unwrap();
                let (wins, rewards) = greedy_bernoulli(runs, iterations, agent_start, pair);
                let greedy = wins
                    .into_iter()
                    .zip(rewards.into_iter())
                    .map(|x| format!("{}, {}", x.0, x.1))
                    .fold(String::from("wins,rewards"), |s, x| [s, x].join("\n"));
                file.write_all(greedy.as_bytes()).unwrap();
            })
        })
    })
}

pub fn pair_epsilon(runs: u32, iterations: u32, agent_start: f64, epsilon: f64) {
    let reward_vec: Vec<f64> = (1..=99).into_iter().map(|x| f64::from(x) / 100.0).collect();
    let pair_vec: Vec<Vec<f64>> = (&reward_vec)
        .into_iter()
        .cartesian_product((&reward_vec).into_iter())
        .filter(|(x, y)| x < y)
        .map(|(x, y)| vec![*x, *y])
        .collect();
    print!("{}", pair_vec.len());
    let mut pool = Pool::new(12);
    pool.scoped(|scope| {
        pair_vec.into_iter().for_each(|pair| {
            scope.execute(move || {
                let mut file = File::create(format!(
                    "results/pair/epsilon_e{}_a{}_{}_{}.csv",
                    epsilon, agent_start, pair[0], pair[1]
                ))
                    .unwrap();
                let (wins, rewards) =
                    epsilon_bernoulli(runs, iterations, agent_start, epsilon, pair);
                let greedy = wins
                    .into_iter()
                    .zip(rewards.into_iter())
                    .map(|x| format!("{}, {}", x.0, x.1))
                    .fold(String::from("wins,rewards"), |s, x| [s, x].join("\n"));
                file.write_all(greedy.as_bytes()).unwrap();
            })
        })
    })
}

pub fn pair_optimistic(runs: u32, iterations: u32, agent_start: f64, c: f64) {
    let reward_vec: Vec<f64> = (1..=99).into_iter().map(|x| f64::from(x) / 100.0).collect();
    let pair_vec: Vec<Vec<f64>> = (&reward_vec)
        .iter()
        .cartesian_product(&reward_vec)
        .filter(|(x, y)| x < y)
        .map(|(x, y)| vec![*x, *y])
        .collect();
    print!("{}", pair_vec.len());
    let mut pool = Pool::new(12);
    pool.scoped(|scope| {
        pair_vec.into_iter().for_each(|pair| {
            scope.execute(move || {
                let mut file = File::create(format!(
                    "results/pair/optimistic_c{}_a{}_{}_{}.csv",
                    c, agent_start, pair[0], pair[1]
                ))
                    .unwrap();
                let (wins, rewards) = optimistic_bernoulli(runs, iterations, agent_start, c, pair);
                let greedy = wins
                    .into_iter()
                    .zip(rewards.into_iter())
                    .map(|x| format!("{}, {}", x.0, x.1))
                    .fold(String::from("wins,rewards"), |s, x| [s, x].join("\n"));
                file.write_all(greedy.as_bytes()).unwrap();
            })
        })
    })
}
