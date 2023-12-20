use ndarray::Array2;
use std::str::FromStr;

#[derive(Clone, Copy)]
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
            _ => Err("Invalid char"),
        }
    }
}
struct Layout {
    mirrors: Array2<Mirror>,
}

impl FromStr for Layout {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirrors = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Mirror::try_from(c).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let nrows = mirrors.len();
        let ncols = mirrors[0].len();
        let mirrors = mirrors.concat();
        let mirrors = Array2::from_shape_vec((nrows, ncols), mirrors).unwrap();
        Ok(Layout { mirrors })
    }
}

pub fn run(input: &str) -> usize {
    let layout = input.parse::<Layout>().unwrap();
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
