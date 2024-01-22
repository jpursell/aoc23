mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 25 a {}", a::run(input));
    println!("day 25 b {}", b::run(input));
}
