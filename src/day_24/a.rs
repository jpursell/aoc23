use std::str::FromStr;

struct Position {
    vec: [usize; 3],
}
struct Velocity {
    vec: [i32; 3],
}
struct InitialCondition {
    position: Position,
    velocity: Velocity,
}
impl FromStr for InitialCondition {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
pub fn run(_input: &str, min_test: usize, max_test: usize) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 7, 27), 2);
    }
}
