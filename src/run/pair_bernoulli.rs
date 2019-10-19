use std::fs::File;
use std::io::Write;

use itertools::Itertools;
use scoped_threadpool::Pool;

use super::greedy_bernoulli;

pub fn pair_greedy(runs: u32, iterations: u32) {
    let reward_vec: Vec<f64> = (1..=9).into_iter().map(|x| f64::from(x) / 10.0).collect();
    let pair_vec: Vec<Vec<f64>> = (&reward_vec).into_iter().cartesian_product((&reward_vec).into_iter())
        .filter(|(x, y)| x < y).map(|(x, y)| vec![*x, *y]).collect();
    let mut pool = Pool::new(12);
    let agent_start = 1.0;
    pool.scoped(|scope| {
        pair_vec.into_iter().for_each(|pair| {
            scope.execute(move || {
                let mut file = File::create(format!("results/pair/pair_{}_{}.csv", pair[0], pair[1])).unwrap();
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