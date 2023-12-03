mod day_1;
use day_1::{day_1a, day_1b};

fn main() {
    let day_1_data = include_str!("day_1_data.txt");
    println!("day 1a {}", day_1a(day_1_data));
    println!("day 1b {}", day_1b(day_1_data));
}