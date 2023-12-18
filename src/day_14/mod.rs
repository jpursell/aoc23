mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 14 a {}", a::run(input));
    println!("day 14 b {}", b::run(input));
}
