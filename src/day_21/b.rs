use std::str::FromStr;

use ndarray::Array2;

#[derive(Debug, Default)]
struct Position {
    index: [usize; 2],
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { index: [row, col] }
    }
}

struct Positions {
    at: Array2<bool>,
}
impl Positions {
    fn step(&self, garden_map: &GardenMap) -> Positions {
        let mut at = Array2::from_elem((garden_map.nrows, garden_map.ncols), false);
        at.indexed_iter_mut().for_each(|((irow, icol), x)| {
            if !garden_map.plot[[irow, icol]] {
                return;
            }
            if irow > 0 {
                if let Some(val) = self.at.get([irow - 1, icol]) {
                    if *val {
                        *x = true;
                        return;
                    }
                }
            }
            if let Some(val) = self.at.get([irow + 1, icol]) {
                if *val {
                    *x = true;
                    return;
                }
            }
            if icol > 0 {
                if let Some(val) = self.at.get([irow, icol - 1]) {
                    if *val {
                        *x = true;
                        return;
                    }
                }
            }
            if let Some(val) = self.at.get([irow, icol + 1]) {
                if *val {
                    *x = true;
                    return;
                }
            }
        });
        Positions { at }
    }
}

impl From<&GardenMap> for Positions {
    fn from(value: &GardenMap) -> Self {
        let mut at = Array2::from_elem((value.nrows, value.ncols), false);
        *at.get_mut(value.start.index).unwrap() = true;
        Positions { at }
    }
}

#[derive(Debug)]
struct GardenMap {
    plot: Array2<bool>,
    start: Position,
    nrows: usize,
    ncols: usize,
}

impl FromStr for GardenMap {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = Position::default();
        'outer: for (irow, line) in s.lines().enumerate() {
            for (icol, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Position::new(irow, icol);
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
            nrows,
            ncols,
        })
    }
}

impl GardenMap {
    fn count_positions(&self, steps: usize) -> usize {
        let mut positions = Positions::from(self);
        for _ in 0..steps {
            positions = positions.step(self);
        }
        positions.at.iter().filter(|x| **x).count()
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
