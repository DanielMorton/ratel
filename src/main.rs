use std::time::Instant;

use ratel::{BinomialBandit, Game, GreedyAgent, HarmonicStepper};

fn main() {
    let rewards = vec![0.1, 0.11, 0.95, 0.13, 0.14, 0.15, 0.16, 0.17, 0.18, 0.19];

    let mut stepper = HarmonicStepper::new(1, rewards.len());
    let mut agent: GreedyAgent = GreedyAgent::new(vec![1.0; rewards.len()], &mut stepper);
    let bandit = BinomialBandit::new(vec![1; rewards.len()], rewards.clone());
    let mut game = Game::new(&mut agent, &bandit);
    let n = 10000u32;
    let r = 100000u32;
    let mut wins = vec![0.0; n as usize];

    let start = Instant::now();
    for _ in 0..=r {
        game.run(n);
        wins = wins.into_iter().zip(game.wins().into_iter())
            .map(|w| w.0 + f64::from(*w.1)).collect();
        game.reset(vec![0.5; rewards.len()])
    }
    wins = wins.into_iter().map(|w| w / f64::from(r)).collect();
    println!("{}", start.elapsed().as_secs());
    println!("{}", wins.last().unwrap())
}
