use std::str::FromStr;

use ndarray::Array2;

enum Mirror {
    N,
    V,
    H,
    S,
    B,
}

impl TryFrom<char> for Mirror {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Mirror::N),
            '|' => Ok(Mirror::V),
            '-' => Ok(Mirror::H),
            '/' => Ok(Mirror::S),
            '\\' => Ok(Mirror::B),
            _ => Err("Invalid char")
        }
    }
}
struct Layout {
    mirrors: Array2<Mirror>,
}
impl FromStr for Layout {
    type Err = &'static;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirrors = s.lines().map()
    }
}
pub fn run(_input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 46);
    }
}
