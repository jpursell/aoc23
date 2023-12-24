pub mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 17 a {}", a::run(input));
    println!("day 17 b {}", b::run(input));
}
