extern crate ratel;

use std::time::Instant;

use ratel::{BinomialBandit, Game, GreedyAgent, HarmonicStepper};

fn main() {
    let rewards = vec![0.77, 0.70, 0.73, 0.53, 0.59, 0.84, 0.42, 0.6, 0.1, 0.3];
    let mut stepper = HarmonicStepper::new(1, rewards.len());
    let mut agent: GreedyAgent = GreedyAgent::new(vec![0.5; rewards.len()], &mut stepper);
    let bandit = BinomialBandit::new(vec![1; rewards.len()], rewards.clone());
    let mut game = Game::new(&mut agent, &bandit);
    let n = 10000u32;

    let start = Instant::now();
    for _ in 0..=1000 {
        game.run(n);
        game.reset(vec![0.5; rewards.len()])
    }
    println!("{}", start.elapsed().as_millis());
}
