mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 10 a {}", a::run(input));
    println!("day 10 b {}", b::run(input));
}
