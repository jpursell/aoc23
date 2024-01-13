mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 23 a {}", a::run(input));
    println!("day 23 b {}", b::run(input));
}
