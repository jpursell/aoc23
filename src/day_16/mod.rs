mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 16 a {}", a::run(input));
    println!("day 16 b {}", b::run(input));
}
