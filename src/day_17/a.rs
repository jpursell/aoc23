use std::str::FromStr;

use ndarray::Array2;

#[derive(Default, Clone, Copy)]
struct Position {
    index: [usize; 2],
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position { index: [row, col] }
    }
    fn row(&self) -> &usize {
        return &self.index[0];
    }
    fn col(&self) -> &usize {
        return &self.index[1];
    }
    fn on_edge(&self, direction: &Direction, loss_map: &LossMap) -> bool {
        match direction {
            Direction::N => *self.row() == 0,
            Direction::S => *self.row() == loss_map.nrows - 1,
            Direction::E => *self.col() == loss_map.ncols - 1,
            Direction::W => *self.col() == 0,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Default)]
enum Direction {
    #[default]
    N,
    E,
    S,
    W,
}

#[derive(Clone, Copy, Default)]
struct Info {
    last_position: Position,
    last_direction: Direction,
    loss: usize,
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
    fn possible_directions(
        &self,
        position: &Position,
        loss_map: &LossMap,
    ) -> [Option<Direction>; 3] {
        let mut out = [None, None, None];
        let mut n = 0;
        let directions = [Direction::N, Direction::E, Direction::S, Direction::W];

        for direction in directions {
            if !position.on_edge(&direction, loss_map)
                && self.table[position.index].last_direction != direction
            {
                out[n] = Some(Direction::N);
                n += 1;
            }
        }

        out
    }
    fn visit(&mut self, position: &Position, loss_map: &LossMap) {
        self.visited[position.index] = true;
        for direction in self.possible_directions(position, loss_map) {
            if direction.is_none() {
                continue;
            }
            let direction = direction.unwrap();
            todo!();
        }
    }
    fn solve(&mut self, loss_map: &LossMap) -> usize {
        self.visit(&Position::new(0, 0));
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
