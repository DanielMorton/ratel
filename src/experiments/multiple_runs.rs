use std::fs::File;
use std::io::Write;

use rand_distr::uniform::Uniform;

use super::{Game, random_init};

pub fn multiple_runs(
    game: &mut Game<u32>,
    pair: &Vec<f64>,
    runs: u32,
    iterations: u32,
    rand_start: &Uniform<f64>,
    file: &mut File,
) {
    let mut wins = vec![0u32; iterations as usize];
    let mut rewards = vec![0u32; iterations as usize];
    for _ in 0..runs {
        game.run(iterations);
        wins = wins
            .iter()
            .zip(game.wins().iter())
            .map(|(w, &gw)| w + gw)
            .collect();
        rewards = rewards
            .iter()
            .zip(game.rewards().iter())
            .map(|(r, &gr)| r + gr)
            .collect();
        let q_new = random_init(rand_start, game.arms());
        game.reset(q_new)
    }
    let output = wins
        .iter()
        .map(|&w| f64::from(w) / f64::from(runs))
        .zip(rewards.iter().map(|&r| f64::from(r) / f64::from(runs)))
        .enumerate()
        .map(|(i, (w, r))|
            format!("{}, {}, {}, {}, {}", pair[0], pair[1], i, w, r))
        .fold(String::from(""), |s, s0| [s, s0].join("\n"));

    file.write_all(output.as_bytes()).unwrap();
}
