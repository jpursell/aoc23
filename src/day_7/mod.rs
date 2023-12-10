mod day_7a;
mod day_7b;

use day_7a::day_7_a;
use day_7b::day_7_b;

pub fn day_7() {
    let input = include_str!("day_7_data.txt");
    println!("day 7 a {}", day_7_a(input));
    println!("day 7 b {}", day_7_b(input));
}
