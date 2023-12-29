mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 21 a {}", a::run(input, 64));
    println!("day 21 b {}", b::run(input));
}
