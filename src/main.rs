use std::fs::File;
use std::io::Write;
use std::time::Instant;

use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::uniform::Uniform;

use ratel::{Agent, BinomialBandit, Game, GreedyAgent, HarmonicStepper};

fn main() {
    let rewards = vec![0.1, 0.11, 0.95, 0.94, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19];

    let mut stepper = HarmonicStepper::new(1, rewards.len());
    let rand_start = Uniform::new(1.0 - 1e-7, 1.0);
    let q_start = (1..=rewards.len()).into_iter().map(|_| rand_start.sample(&mut thread_rng())).collect();
    let mut agent: GreedyAgent = GreedyAgent::new(q_start, &mut stepper);
    let bandit = BinomialBandit::new(vec![1; rewards.len()], rewards.clone());
    let mut game = Game::new(&mut agent, &bandit);
    let n = 100u32;
    let r = 1000000u32;
    let mut wins = vec![0.0; n as usize];
    let mut reward_out = vec![0.0; n as usize];

    let start = Instant::now();
    for _ in 0..=r {
        game.run(n);
        wins = wins.into_iter().zip(game.wins().into_iter())
            .map(|w| w.0 + f64::from(*w.1)).collect();
        reward_out = reward_out.into_iter().zip(game.rewards().into_iter())
            .map(|ro| ro.0 + f64::from(*ro.1)).collect();
        game.reset((1..=rewards.len()).into_iter()
            .map(|_| rand_start.sample(&mut thread_rng())).collect())
    }
    wins = wins.into_iter().map(|w| w / f64::from(r)).collect();
    reward_out = reward_out.into_iter().map(|ro| ro / f64::from(r)).collect();
    let greedy = wins.into_iter().zip(reward_out.into_iter())
        .map(|x| format!("{}, {}", x.0, x.1))
        .fold(String::from("wins, rewards"), |s, x| [s, x].join("\n"));
    let mut file = File::create("greedy.csv").unwrap();
    file.write_all(greedy.as_bytes());

    println!("{}", start.elapsed().as_secs());
}
