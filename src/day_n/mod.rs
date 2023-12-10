mod a;
mod b;

use a::run_a;
use b::run_b;

pub fn day_n() {
    let input = include_str!("data.txt");
    println!("day n a {}", run_a(input));
    println!("day n b {}", run_b(input));
}
