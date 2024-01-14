mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!(
        "day 24 a {}",
        a::run(input, 200_000_000_000_000, 400_000_000_000_000)
    );
    println!("day 24 b {}", b::run(input));
}
