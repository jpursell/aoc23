mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 20 a {}", a::run(input));
    println!("day 20 b {}", b::run(input));
}
