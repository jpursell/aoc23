mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day 19 a {}", a::run(input));
    println!("day 19 b {}", b::run(input));
}
