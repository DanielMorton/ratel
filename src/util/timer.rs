use std::time::Instant;

pub fn print_hms(start: Instant) -> () {
    let seconds = start.elapsed().as_secs();
    let (hour, minute, second) = (seconds / 3600, (seconds % 3600) / 60, seconds % 60);
    println!(
        "{}:{}:{}",
        hour,
        format!(if minute < 10 { "0{}" } else { "{}" }, minute),
        format!(if second < 10 { "0{}" } else { "{}" }, second)
    )
}
