mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 13 a {}", a::run(input));
    println!("day 13 b {}", b::run(input));
}
