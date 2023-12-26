mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 18 a {}", a::run(input));
    println!("day 18 b {}", b::run(input));
}
