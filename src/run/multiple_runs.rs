use std::fs::File;
use std::io::Write;
use std::ops::AddAssign;

use rand_distr::uniform::Uniform;

use super::{random_init, Game};

pub fn multiple_runs(
    game: &mut Game<u32>,
    runs: u32,
    iterations: u32,
    rand_start: &Uniform<f64>,
    file_name: String,
) {
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
        let q_new = random_init(rand_start, game.arms());
        game.reset(q_new)
    }
    let output = wins
        .iter()
        .map(|&w| f64::from(w) / f64::from(runs))
        .zip(rewards.iter().map(|&r| f64::from(r) / f64::from(runs)))
        .map(|(w, r)| format!("{}, {}", w, r))
        .fold(String::from("wins,rewards"), |s, s0| [s, s0].join("\n"));
    let mut file = File::create(file_name).unwrap();
    file.write_all(output.as_bytes()).unwrap();
}
