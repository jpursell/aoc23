mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 9 a {}", a::run(input));
    println!("day 9 b {}", b::run(input));
}
