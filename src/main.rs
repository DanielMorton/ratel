extern crate scoped_threadpool;

use std::time::Instant;

use scoped_threadpool::Pool;

use ratel::{epsilon_bernoulli, greedy_bernoulli};

fn main() {
    let n = 10000u32;
    let r = 10000u32;
    let mut pool = Pool::new(12);
    let vec: Vec<u32> = (1..=100).into_iter().map(|x| x).collect();
    let start = Instant::now();
    pool.scoped(|scope| {
        for x in vec {
            scope.execute(move || epsilon_bernoulli(r, n, f64::from(x) / 100.0, 0.1))
        }
    });
    println!("{}", start.elapsed().as_secs());
}
