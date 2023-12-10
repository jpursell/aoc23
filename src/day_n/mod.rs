mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!("day n a {}", a::run(input));
    println!("day n b {}", b::run(input));
}
