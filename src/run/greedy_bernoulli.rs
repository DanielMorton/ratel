use rand::distributions::Distribution;
use rand::thread_rng;
use rand_distr::uniform::Uniform;

use super::{BinomialBandit, Game, GreedyAgent, HarmonicStepper};

pub fn greedy_bernoulli(
    runs: u32,
    iterations: u32,
    agent_start: f64,
    rewards: Vec<f64>,
) -> (Vec<f64>, Vec<f64>) {
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
    let mut wins = vec![0u32; iterations as usize];
    let mut reward_out = vec![0u32; iterations as usize];

    for _ in 0..runs {
        game.run(iterations);
        wins = wins
            .into_iter()
            .zip(game.wins().into_iter())
            .map(|(w, gw)| w + *gw)
            .collect();
        reward_out = reward_out
            .into_iter()
            .zip(game.rewards().into_iter())
            .map(|(r, gr)| r + *gr)
            .collect();
        game.reset(
            (1..=rewards.len())
                .into_iter()
                .map(|_| rand_start.sample(&mut thread_rng()))
                .collect(),
        )
    }
    (
        wins.into_iter()
            .map(|w| f64::from(w) / f64::from(runs))
            .collect(),
        reward_out
            .into_iter()
            .map(|ro| f64::from(ro) / f64::from(runs))
            .collect(),
    )
}
