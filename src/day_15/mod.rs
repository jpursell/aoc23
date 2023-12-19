mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 15 a {}", a::run(input));
    println!("day 15 b {}", b::run(input));
}
