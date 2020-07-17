use std::time::Instant;

use clap::{App, Arg, value_t};

use ratel::{pool_bernoulli, print_hms};

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
                .long("pair-greedy")
                .help("Use the greedy algorithm with two-armed bandits")
                .conflicts_with("greedy")
                .conflicts_with("epsilon_greedy"),
        )
        .arg(
            Arg::with_name("pair_epsilon")
                .long("pair-epsilon")
                .help("Use the epsilon-greedy algorithm with two-armed bandits")
                .takes_value(true)
                .conflicts_with("greedy")
                .conflicts_with("epsilon_greedy")
                .conflicts_with("pair_greedy"),
        )
        .arg(
            Arg::with_name("pair_optimistic")
                .long("pair-optimistic")
                .help("Use the optimistic algorithm with two-armed bandits")
                .takes_value(true)
                .conflicts_with("greedy")
                .conflicts_with("epsilon_greedy")
                .conflicts_with("pair_greedy")
                .conflicts_with("pair_epsilon"),
        )
        .get_matches();
    let runs = value_t!(matches.value_of("runs"), u32).unwrap_or_else(|e| e.exit());
    let iterations = value_t!(matches.value_of("iterations"), u32).unwrap_or_else(|e| e.exit());
    if matches.is_present("greedy") || matches.is_present("epsilon_greedy") {
        //    run_bernoulli(runs, iterations, &matches)
    } else if matches.is_present("pair_greedy")
        || matches.is_present("pair_epsilon")
        || matches.is_present("pair_optimistic")
    {
        let start = Instant::now();
        let agent_start: Vec<f64> = (1..=10).map(|x| f64::from(x) / 10.0).collect();
        pool_bernoulli(runs, iterations, &agent_start, &matches);
        print_hms(start);
    }
}

// fn run_bernoulli(runs: u32, iterations: u32, arg: &ArgMatches) {
//     let mut pool = Pool::new(12);
//     let vec: Vec<f64> = (1..=100)
//         .map(|x| f64::from(x) / 100.0)
//         .collect();
//     let start = Instant::now();
//     pool.scoped(|scope| {
//         for x in vec {
//             scope.execute(move || sequential_bernoulli(runs, iterations, x, arg))
//         }
//     });
//     print_hms(start);
// }
