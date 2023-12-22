use std::str::FromStr;

use ndarray::Array2;

#[derive(Debug, Default, Clone, Copy)]
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
    fn on_edge(&self, direction: &Direction, loss_map: &LossMap, distance: &usize) -> bool {
        match direction {
            Direction::N => *self.row() <= distance - 1,
            Direction::S => *self.row() >= loss_map.nrows - distance,
            Direction::E => *self.col() >= loss_map.ncols - distance,
            Direction::W => *self.col() <= distance - 1,
        }
    }
    /// Return new position at new location
    ///
    /// Should have already checked this is ok
    fn move_by(&self, distance: &usize, direction: &Direction) -> Position {
        match direction {
            Direction::N => Position::new(*self.row() - distance, *self.col()),
            Direction::E => Position::new(*self.row(), *self.col() + distance),
            Direction::S => Position::new(*self.row() + distance, *self.col()),
            Direction::W => Position::new(*self.row(), *self.col() - distance),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, Default)]
enum Direction {
    #[default]
    N,
    E,
    S,
    W,
}

#[derive(Debug, Clone, Copy)]
struct Info {
    last_position: Position,
    last_direction: Direction,
    loss: usize,
}
impl Default for Info {
    fn default() -> Self {
        Info {
            last_position: Position::default(),
            last_direction: Direction::default(),
            loss: usize::MAX,
        }
    }
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
        let mut table = Array2::<Info>::from_elem((nrows, ncols), Info::default());
        table[[0, 0]].loss = 0;

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
    fn move_possible(
        &self,
        position: &Position,
        distance: &usize,
        direction: &Direction,
        loss_map: &LossMap,
    ) -> bool {
        !position.on_edge(&direction, loss_map, distance)
            && self.table[position.index].last_direction != *direction
    }
    /// Visit node
    /// Node will become added to visited list
    /// All nodes connected will be updated in table
    fn visit(&mut self, position: &Position, loss_map: &LossMap) {
        self.visited[position.index] = true;
        // todo refactor possible_directions to just check a single direction
        // so I can rewrite this double loop with direction on the outside and
        // distance on the inside so I can accumulate the heat loss
        for direction in [Direction::N, Direction::E, Direction::S, Direction::W] {
            let mut loss: usize = 0;
            for distance in 1..=3 {
                if self.move_possible(position, &distance, &direction, loss_map) {
                    let new_position = position.move_by(&distance, &direction);
                    if self.visited[new_position.index] {
                        continue;
                    }
                    loss += loss_map.data[new_position.index] as usize;
                    let entry = self.table.get_mut(new_position.index).unwrap();
                    if entry.loss > loss {
                        entry.loss = loss;
                        entry.last_direction = direction;
                        entry.last_position = *position;
                        dbg!(&new_position);
                        dbg!(&entry);
                    }
                }
            }
        }
    }

    /// Solve and return lowest heat loss
    fn solve(&mut self, loss_map: &LossMap) -> usize {
        self.visit(&Position::new(0, 0), loss_map);
        // todo: loop through unvisited nodes and visit them
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
