use std::fs::File;
use std::io::Write;

use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::uniform::Uniform;

use super::{BinomialBandit, Game, GreedyAgent, HarmonicStepper};

pub fn greedy_bernoulli(runs: u32, iterations: u32, agent_start: f64) {
    let rewards = vec![0.1, 0.11, 0.95, 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19];

    let mut stepper = HarmonicStepper::new(1, rewards.len());
    let rand_start = Uniform::new(agent_start - 1e-7, agent_start + 1e-7);
    let mut agent = GreedyAgent::new(
        (1..=rewards.len())
            .into_iter()
            .map(|_| rand_start.sample(&mut thread_rng()))
            .collect(),
        &mut stepper,
    );
    let bandit = BinomialBandit::new(vec![1; rewards.len()], rewards.clone());
    let mut game = Game::new(&mut agent, &bandit);
    let mut wins = vec![0.0; iterations as usize];
    let mut reward_out = vec![0.0; iterations as usize];

    for _ in 0..=runs {
        game.run(iterations);
        wins = wins
            .into_iter()
            .zip(game.wins().into_iter())
            .map(|w| w.0 + f64::from(*w.1))
            .collect();
        reward_out = reward_out
            .into_iter()
            .zip(game.rewards().into_iter())
            .map(|ro| ro.0 + f64::from(*ro.1))
            .collect();
        game.reset(
            (1..=rewards.len())
                .into_iter()
                .map(|_| rand_start.sample(&mut thread_rng()))
                .collect(),
        )
    }
    wins = wins.into_iter().map(|w| w / f64::from(runs)).collect();
    reward_out = reward_out
        .into_iter()
        .map(|ro| ro / f64::from(runs))
        .collect();
    let greedy = wins
        .into_iter()
        .zip(reward_out.into_iter())
        .map(|x| format!("{}, {}", x.0, x.1))
        .fold(String::from("wins,rewards"), |s, x| [s, x].join("\n"));
    let mut file = File::create(format!("greedy/greedy_{}.csv", agent_start)).unwrap();
    file.write_all(greedy.as_bytes()).unwrap();
}