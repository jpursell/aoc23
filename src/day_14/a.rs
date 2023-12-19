use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
enum Rock {
    R,
    S,
    N,
}
impl TryFrom<char> for Rock {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Rock::R),
            '.' => Ok(Rock::N),
            '#' => Ok(Rock::S),
            _ => Err("Unknown Rock"),
        }
    }
}
struct RockField {
    rocks: Vec<Vec<Rock>>,
}
impl FromStr for RockField {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Rock::try_from(c).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(RockField { rocks })
    }
}
#[derive(Debug, PartialEq, Eq)]
enum Direction {
    N,
    // E,
    // S,
    // W,
}
impl RockField {
    fn roll(&mut self, direction: Direction) -> bool {
        assert_eq!(direction, Direction::N);
        let mut changed = false;
        for irow in 1..self.rocks.len() {
            for icol in 0..self.rocks[0].len() {
                if self.rocks[irow][icol] == Rock::R && self.rocks[irow - 1][icol] == Rock::N {
                    self.rocks[irow][icol] = Rock::N;
                    self.rocks[irow - 1][icol] = Rock::R;
                    changed = true
                }
            }
        }
        changed
    }
    fn count_rocks(&self) -> usize {
        let nrows = self.rocks.len();
        self.rocks
            .iter()
            .enumerate()
            .map(|(irow, row)| {
                row.iter()
                    .map(|r| if *r == Rock::R { nrows - irow } else { 0 })
                    .sum::<usize>()
            })
            .sum::<usize>()
    }
}
pub fn run(input: &str) -> usize {
    let mut field = input.parse::<RockField>().unwrap();
    loop {
        if !field.roll(Direction::N) {
            break;
        }
    }
    field.count_rocks()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 136);
    }
}
