use std::{collections::BTreeSet, str::FromStr};

use ndarray::Array2;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
struct Position {
    index: [i64; 2],
}

impl Position {
    fn new(row: i64, col: i64) -> Position {
        Position { index: [row, col] }
    }
    fn row(&self) -> i64 {
        self.index[0]
    }
    fn col(&self) -> i64 {
        self.index[1]
    }
    fn neighbors(&self) -> [Position; 4] {
        let row = self.row();
        let col = self.col();
        [
            Position::new(row - 1, col),
            Position::new(row + 1, col),
            Position::new(row, col + 1),
            Position::new(row, col - 1),
        ]
    }
}

struct Positions {
    at: BTreeSet<Position>,
}
impl Positions {
    fn step(&self, garden_map: &GardenMap) -> Positions {
        let mut at = BTreeSet::new();
        self.at.iter().for_each(|p| {
            for new_pos in p.neighbors() {
                if garden_map.on_plot(&new_pos) {
                    at.insert(new_pos);
                }
            }
        });
        Positions { at }
    }
}

impl From<&GardenMap> for Positions {
    fn from(value: &GardenMap) -> Self {
        let mut at = BTreeSet::new();
        at.insert(value.start);
        Positions { at }
    }
}

#[derive(Debug)]
struct GardenMap {
    plot: Array2<bool>,
    start: Position,
    nrows: i64,
    ncols: i64,
}

impl FromStr for GardenMap {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Position::default();
        'outer: for (irow, line) in s.lines().enumerate() {
            for (icol, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Position::new(irow as i64, icol as i64);
                    break 'outer;
                }
            }
        }
        let plot = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'S' => true,
                        '.' => true,
                        '#' => false,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let nrows = plot.len();
        let ncols = plot[0].len();
        let plot = Array2::from_shape_vec((nrows, ncols), plot.concat()).unwrap();
        Ok(GardenMap {
            plot,
            start,
            nrows: nrows as i64,
            ncols: ncols as i64,
        })
    }
}

impl GardenMap {
    fn count_positions(&self, steps: usize) -> usize {
        let mut positions = Positions::from(self);
        for _ in 0..steps {
            positions = positions.step(self);
        }
        positions.at.len()
    }
    fn on_plot(&self, position: &Position) -> bool {
        let mut row = position.row();
        let mut col = position.col();
        while row < 0 {
            row += self.nrows;
        }
        while col < 0 {
            col += self.ncols;
        }
        while row >= self.nrows {
            row -= self.nrows;
        }
        while col >= self.ncols {
            col -= self.ncols;
        }
        self.plot[[row as usize, col as usize]]
    }
}

pub fn run(input: &str, steps: usize) -> usize {
    input.parse::<GardenMap>().unwrap().count_positions(steps)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 6), 16);
    }
    #[test]
    fn test2() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 10), 50);
    }
    #[test]
    fn test3() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 50), 1594);
    }
    #[test]
    fn test4() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 100), 6536);
    }
    #[test]
    fn test5() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 500), 167004);
    }
    #[test]
    fn test6() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 1000), 668697);
    }
    #[test]
    fn test7() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 5000), 16733044);
    }
}
