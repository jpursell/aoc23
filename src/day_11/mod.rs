mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 11 a {}", a::run(input));
    println!("day 11 b {}", b::run(input));
}
