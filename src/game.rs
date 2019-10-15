//use super::{Agent, Bandit, Counter, RecordCounter};
//use num_traits::{Num, ToPrimitive};
//
//struct Game<'a, T: Num + ToPrimitive> {
//    agent: &'a dyn Agent<T>,
//    bandit: &'a dyn Bandit<T>,
//    wins: RecordCounter<u32>,
//    rewards: RecordCounter<T>
//}
//
//impl<'a, T: Num + ToPrimitive> Game<'a, T> {
//    fn new(agent: &'a dyn Agent<T>, bandit: &'a dyn Bandit<T>) -> Game<'a, T> {
//        Game {agent, bandit, wins: RecordCounter::new(), rewards: RecordCounter::new()}
//    }
//
//    fn pull_arm(&mut self) -> () {
//        let current_action = self.agent.action();
//        self.wins.update((current_action == self.bandit.best_arm()) as u32);
//        let reward = self.bandit.reward(&current_action);
//        self.rewards.update(&reward)
//    }
//}