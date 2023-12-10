mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 8 a {}", a::run(input));
    println!("day 8 b {}", b::run(input));
}
