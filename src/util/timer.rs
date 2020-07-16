use std::time::Instant;

/// Prints the amount of time that has elapsed since a timer was started.
pub fn print_hms(start: Instant) -> () {
    let seconds = start.elapsed().as_secs();
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!(
        "{}:{}:{}",
        hour,
        if minute < 10 {
            format!("0{}", minute)
        } else {
            format!("{}", minute)
        },
        if second < 10 {
            format!("0{}", second)
        } else {
            format!("{}", second)
        }
    )
}
