use std::collections::HashMap;
use std::ops::AddAssign;

use num_traits::{Num, ToPrimitive};

use super::{Agent, Bandit, Counter, RecordCounter};

struct Game<'a, T: AddAssign + Num + ToPrimitive> {
    agent: &'a mut dyn Agent<T>,
    bandit: &'a dyn Bandit<T>,
    wins: RecordCounter<u32>,
    rewards: RecordCounter<T>,
}

impl<'a, T: AddAssign + Copy + Num + ToPrimitive> Game<'a, T> {
    fn new(agent: &'a mut dyn Agent<T>, bandit: &'a dyn Bandit<T>) -> Game<'a, T> {
        Game {
            agent,
            bandit,
            wins: RecordCounter::new(),
            rewards: RecordCounter::new(),
        }
    }

    fn pull_arm(&mut self) -> () {
        let current_action = self.agent.action();
        self.wins
            .update((current_action == self.bandit.best_arm()) as u32);
        let reward = self.bandit.reward(current_action);
        self.rewards.update(reward);
        self.agent.step(current_action, reward)
    }

    fn reset(&mut self, q_init: Vec<f64>) -> () {
        self.agent.reset(q_init);
        self.rewards.reset();
        self.wins.reset();
    }

    fn rewards(&self) -> &RecordCounter<T> {
        &self.rewards
    }

    fn wins(&self) -> &RecordCounter<u32> {
        &self.wins
    }
}
