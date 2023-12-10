mod day_na;
mod day_nb;

use day_na::day_n_a;
use day_nb::day_n_b;

pub fn day_n() {
    let input = include_str!("day_n_data.txt");
    println!("day n a {}", day_n_a(input));
    println!("day n b {}", day_n_b(input));
}
