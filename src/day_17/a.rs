use std::str::FromStr;

use ndarray::Array2;

#[derive(Default, Clone, Copy)]
struct Position {
    row: u8,
    col: u8,
}
#[derive(Clone, Copy, Default)]
struct Info {
    last_position: Position,
    last_straight: u8,
    loss: u16,
}
struct Solver {
    visited: Array2<bool>,
    table: Array2<Info>,
    nrows: usize,
    ncols: usize,
}

impl From<&LossMap> for Solver {
    fn from(loss_map: &LossMap) -> Self {
        let (nrows, ncols) = (loss_map.nrows, loss_map.ncols);
        let visited = Array2::<bool>::from_elem((nrows, ncols), false);
        let table = Array2::<Info>::from_elem((nrows, ncols), Info::default());
        Solver {
            visited,
            table,
            nrows,
            ncols,
        }
    }
}

struct LossMap {
    data: Array2<u8>,
    nrows: usize,
    ncols: usize,
}

impl FromStr for LossMap {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let nrows = data.len();
        let ncols = data[0].len();
        let data = Array2::from_shape_vec((nrows, ncols), data.concat()).unwrap();
        Ok(LossMap {
            data,
            nrows: nrows,
            ncols: ncols,
        })
    }
}

impl Solver {
    fn visit(&mut self, row:usize,col:usize) {
        self.visited[[row,col]] = true;
    }
    fn solve(&mut self, loss_map: &LossMap) -> usize {
        todo!()
        self.table[[0, 0]].
        self.visit(0, 0);
        todo!()
    }
}

pub fn run(input: &str) -> usize {
    let loss_map = input.parse::<LossMap>().unwrap();
    let mut solver = Solver::from(&loss_map);
    solver.solve(&loss_map)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 102);
    }
}
