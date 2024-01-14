mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 24 a {}", a::run(input));
    println!("day 24 b {}", b::run(input));
}
