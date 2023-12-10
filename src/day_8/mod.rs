mod a;
mod b;

use a::run_a;
use b::run_b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 8 a {}", run_a(input));
    println!("day 8 b {}", run_b(input));
}
