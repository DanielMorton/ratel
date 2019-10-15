extern crate ratel;

use std::time::Instant;

use ratel::{BinomialBandit, Game, GreedyAgent, HarmonicStepper};

fn main() {
    let rewards = vec![0.05, 0.15, 0.25, 0.35, 0.45, 0.55, 0.65, 0.75, 0.85, 0.95];
    let mut stepper = HarmonicStepper::new(1, rewards.len());
    let mut agent: GreedyAgent = GreedyAgent::new(vec![0.5; rewards.len()], &mut stepper);
    let bandit = BinomialBandit::new(vec![1; rewards.len()], rewards);
    let mut game = Game::new(&mut agent, &bandit);
    let n = 10000u32;

    let start = Instant::now();
    game.run(n);
    println!("{}", start.elapsed().as_millis());
}
