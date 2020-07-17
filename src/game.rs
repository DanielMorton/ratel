use std::ops::AddAssign;

use num_traits::{Num, ToPrimitive};

use super::{Agent, Bandit, Counter, RecordCounter};

///Structure to make the Agent interact with the Bandit.
pub struct Game<'a, T: AddAssign + Num + ToPrimitive> {
    /// Agent learning about bandit.
    agent: &'a mut dyn Agent<T>,
    ///Bandit used by agent.
    bandit: &'a dyn Bandit<T>,
    /// Records wins and losses from each arm pull. Win means pulling the best arm.
    wins: RecordCounter<u32>,
    /// Records rewards from each arm pull.
    rewards: RecordCounter<T>,
}

impl<'a, T: AddAssign + Copy + Num + ToPrimitive> Game<'a, T> {
    /// Initializes a Game with an Agent, Bandit, and new counters.
    pub fn new(agent: &'a mut dyn Agent<T>, bandit: &'a dyn Bandit<T>) -> Game<'a, T> {
        assert_eq!(agent.arms(), bandit.arms());
        Game {
            agent,
            bandit,
            wins: RecordCounter::new(),
            rewards: RecordCounter::new(),
        }
    }

    /// Returns the number of bandit arms.
    pub fn arms(&self) -> usize {
        self.bandit.arms()
    }

    /// Agent chooses an arm to pull and updates based on reward.
    fn pull_arm(&mut self) {
        let current_action = self.agent.action();
        self.wins
            .update((current_action == self.bandit.best_arm()) as u32);
        let reward = self.bandit.reward(current_action);
        self.rewards.update(reward);
        self.agent.step(current_action, reward);
    }

    /// Resets Game. Resets Agent with new initial guess and resets counters.
    pub fn reset(&mut self, q_init: Vec<f64>) {
        self.agent.reset(q_init);
        self.rewards.reset();
        self.wins.reset();
    }

    /// Returns vector of rewards.
    pub fn rewards(&self) -> &Vec<T> {
        self.rewards.record()
    }

    /// Run game for a certain number of steps.
    pub fn run(&mut self, steps: u32) {
        for _ in 1..=steps {
            self.pull_arm()
        }
    }

    /// Returns vector of wins.
    pub fn wins(&self) -> &Vec<u32> {
        self.wins.record()
    }
}
