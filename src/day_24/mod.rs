mod a;
mod b;

pub fn run() {
    let input = include_str!("data.txt");
    println!(
        "day 24 a {}",
        a::run(input, 200_000_000_000_000_f64, 400_000_000_000_000_f64)
    );
    println!("day 24 b {}", b::run(input));
}
