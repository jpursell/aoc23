use self::b::RunMode;

mod a;
pub mod b;

pub fn run(mode: RunMode) {
    let input = include_str!("data.txt");
    println!("day 12 a {}", a::run(input));
    println!("day 12 b {}", b::run(input, mode));
}
