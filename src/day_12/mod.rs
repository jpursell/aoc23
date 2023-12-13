mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 12 a {}", a::run(input));
    println!("day 12 b {}", b::run(input));
}
