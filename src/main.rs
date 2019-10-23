use std::time::Instant;

use clap::{App, Arg, value_t};
use scoped_threadpool::Pool;

use ratel::{pair_epsilon, pair_greedy, sequential_bernoulli};

fn main() {
    let matches = App::new("Ratel")
        .arg(
            Arg::with_name("runs")
                .short("r")
                .help("Number of trial runs.")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("iterations")
                .short("n")
                .help("Number of iterations in each trial")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("greedy")
                .short("g")
                .long("greedy")
                .help("Use the greedy algorithm."),
        )
        .arg(
            Arg::with_name("epsilon_greedy")
                .short("e")
                .help("Use the epsilon-greedy algorithm")
                .takes_value(true)
                .conflicts_with("greedy"),
        )
        .arg(
            Arg::with_name("pair_greedy")
                .short("pg")
                .long("pair-greedy")
                .help("Use the greedy algorithm with two-armed bandits")
                .conflicts_with("greedy")
                .conflicts_with("epsilon_greedy"),
        )
        .arg(
            Arg::with_name("pair_epsilon")
                .short("pe")
                .long("pair-epsilon")
                .help("Use the epsilon-greedy algorithm with two-armed bandits")
                .takes_value(true)
                .conflicts_with("greedy")
                .conflicts_with("epsilon_greedy")
                .conflicts_with("pair_greedy"),
        )
        .get_matches();
    let runs = value_t!(matches.value_of("runs"), u32).unwrap_or_else(|e| e.exit());
    let iterations = value_t!(matches.value_of("iterations"), u32).unwrap_or_else(|e| e.exit());
    if matches.is_present("greedy") {
        run_greedy(runs, iterations)
    } else if matches.is_present("epsilon_greedy") {
        let epsilon =
            value_t!(matches.value_of("epsilon_greedy"), f64).unwrap_or_else(|e| e.exit());
        run_epsilon(runs, iterations, epsilon)
    } else if matches.is_present("pair_greedy") {
        let start = Instant::now();
        pair_greedy(runs, iterations);
        println!("{}", start.elapsed().as_secs());
    } else if matches.is_present("pair_epsilon") {
        let epsilon = value_t!(matches.value_of("pair_epsilon"), f64).unwrap_or_else(|e| e.exit());
        let start = Instant::now();
        pair_epsilon(runs, iterations, epsilon);
        println!("{}", start.elapsed().as_secs());
    }
}

fn run_epsilon(runs: u32, iterations: u32, epsilon: f64) {
    let mut pool = Pool::new(12);
    let vec: Vec<u32> = (1..=100).into_iter().map(|x| x).collect();
    let start = Instant::now();
    pool.scoped(|scope| {
        for x in vec {
            scope.execute(move || {
                sequential_bernoulli(runs, iterations, f64::from(x) / 100.0, epsilon)
            })
        }
    });
    println!("{}", start.elapsed().as_secs());
}

fn run_greedy(runs: u32, iterations: u32) {
    let mut pool = Pool::new(12);
    let int_vec: Vec<u32> = (1..=100).into_iter().map(|x| x).collect();
    let start = Instant::now();
    pool.scoped(|scope| {
        for x in int_vec {
            scope
                .execute(move || sequential_bernoulli(runs, iterations, f64::from(x) / 100.0, 0.0));
        }
    });
    println!("{}", start.elapsed().as_secs());
}
