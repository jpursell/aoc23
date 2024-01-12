mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 22 a {}", a::run(input));
    println!("day 22 b {}", b::run(input));
}
