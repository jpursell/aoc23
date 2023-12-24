use ndarray::{Array2, Array3};
use std::collections::BTreeSet;
use std::fmt::Display;
use std::str::FromStr;
use std::time::Instant;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Default, Clone, Copy)]
struct Position {
    index: [usize; 2],
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row(), self.col())
    }
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
    fn dindex(&self, direction: &Direction) -> [usize; 3] {
        [*self.row(), *self.col(), usize::from(*direction)]
    }
    fn on_edge(&self, direction: &Direction, loss_map: &LossMap, distance: &usize) -> bool {
        match direction {
            Direction::N => *self.row() <= distance - 1,
            Direction::S => {
                if loss_map.nrows < *distance {
                    true
                } else {
                    *self.row() >= loss_map.nrows - distance
                }
            }
            Direction::E => {
                if loss_map.ncols < *distance {
                    true
                } else {
                    *self.col() >= loss_map.ncols - distance
                }
            }
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
    fn move_possible(&self, distance: &usize, direction: &Direction, loss_map: &LossMap) -> bool {
        !self.on_edge(&direction, loss_map, distance)
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

impl Direction {
    fn all_but(direction: &Direction) -> [Direction; 3] {
        match direction {
            Direction::N => [Direction::E, Direction::S, Direction::W],
            Direction::E => [Direction::N, Direction::S, Direction::W],
            Direction::S => [Direction::N, Direction::E, Direction::W],
            Direction::W => [Direction::N, Direction::E, Direction::S],
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::N => write!(f, "N"),
            Direction::E => write!(f, "E"),
            Direction::S => write!(f, "S"),
            Direction::W => write!(f, "W"),
        }
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::N => 0,
            Direction::E => 1,
            Direction::S => 2,
            Direction::W => 3,
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = &'static str;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::N),
            1 => Ok(Direction::E),
            2 => Ok(Direction::S),
            3 => Ok(Direction::W),
            _ => Err("Unexpected value"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Info {
    last_position: Position,
    last_direction: Direction,
    loss: usize,
}

impl Display for Info {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ last_position: {} last_direction {} loss {} ]",
            self.last_position, self.last_direction, self.loss
        )
    }
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
    table: Array3<Info>,
    nrows: usize,
    ncols: usize,
}

impl From<&LossMap> for Solver {
    fn from(loss_map: &LossMap) -> Self {
        let (nrows, ncols) = (loss_map.nrows, loss_map.ncols);
        let visited = Array2::<bool>::from_elem((nrows, ncols), false);
        let mut table = Array3::<Info>::from_elem((nrows, ncols, 4), Info::default());
        for direction in [Direction::S, Direction::W] {
            table[[0, 0, direction.into()]].loss = 0;
        }

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
    /// Visit node
    /// Node will become added to visited list
    /// All nodes connected will be updated in table
    fn visit(&mut self, position: &Position, loss_map: &LossMap) {
        self.visited[position.index] = true;
        let directions = [Direction::N, Direction::E, Direction::S, Direction::W];
        for direction in directions {
            let entry = directions
                .iter()
                .filter(|d| **d != direction)
                .map(|d| self.table[[*position.row(), *position.col(), usize::from(*d)]])
                .min_by_key(|e| e.loss)
                .unwrap();
            let mut loss: usize = entry.loss;
            if loss == usize::MAX {
                continue;
            }
            for distance in 1..=3 {
                if position.move_possible(&distance, &direction, loss_map) {
                    let new_position = position.move_by(&distance, &direction);
                    if self.visited[new_position.index] {
                        continue;
                    }
                    loss += loss_map.data[new_position.index] as usize;
                    let entry = self.table.get_mut(new_position.dindex(&direction)).unwrap();
                    if entry.loss > loss {
                        // println!(
                        //     "entry for new position {} and dir {} {} loss > {}",
                        //     new_position, direction, entry, loss
                        // );
                        entry.loss = loss;
                        entry.last_direction = direction;
                        entry.last_position = *position;
                        // println!(
                        //     "visit pos {} set new pos {} loss to {} moving in dir {}",
                        //     position, new_position, loss, direction
                        // );
                    }
                }
            }
        }
    }

    fn find_next_node(&self) -> Option<Position> {
        let mut found = false;
        let mut position = Position::default();
        let mut loss = usize::MAX;
        self.table
            .indexed_iter()
            .for_each(|((row, col, _), entry)| {
                if !self.visited[[row, col]] && entry.loss < loss {
                    found = true;
                    position = Position::new(row, col);
                    loss = entry.loss
                }
            });
        if found {
            Some(position)
        } else {
            None
        }
    }

    fn print_trace(&self, position: &Position, loss_map: &LossMap) {
        let mut trace = Array2::from_elem((self.nrows * 2 + 1, self.ncols * 2 + 1), ' ');
        loss_map.data.indexed_iter().for_each(|((r, c), x)| {
            *trace.get_mut([r * 2 + 1, c * 2 + 1]).unwrap() =
                format!("{}", x).chars().next().unwrap();
        });
        let mut current = position;
        let directions = [Direction::N, Direction::E, Direction::S, Direction::W];
        let mut direction = &Direction::W;
        loop {
            match direction {
                Direction::W => {
                    trace[[*current.row() * 2 + 1, *current.col() * 2]] = '#';
                }
                Direction::N => {
                    trace[[*current.row() * 2, *current.col() * 2 + 1]] = '#';
                }
                Direction::E => {
                    trace[[*current.row() * 2 + 1, *current.col() * 2 + 2]] = '#';
                }
                Direction::S => {
                    trace[[*current.row() * 2 + 2, *current.col() * 2 + 1]] = '#';
                }
            }
            let entry = directions
                .iter()
                .filter(|d| **d != *direction)
                .map(|d| &self.table[current.dindex(d)])
                .min_by_key(|e| e.loss)
                .unwrap();
            if entry.loss == 0 {
                break;
            }
            let row_diff = (*current.row()).abs_diff(*entry.last_position.row());
            let col_diff = (*current.col()).abs_diff(*entry.last_position.col());
            assert!(row_diff == 0 || col_diff == 0);
            assert!(row_diff.max(col_diff) > 0);
            assert!(row_diff.max(col_diff) <= 3);
            current = &entry.last_position;
            direction = &entry.last_direction;
            // println!("{:?} {:?}", current, direction);
        }
        for row in 0..(self.nrows * 2 + 1) {
            if row % 2 == 1 {
                print!("{:03} ", row / 2);
            } else {
                print!("    ");
            }
            for col in 0..(self.ncols * 2 + 1) {
                print!("{}", trace[[row, col]]);
            }
            println!("");
        }
    }

    /// Solve and return lowest heat loss
    fn solve(&mut self, loss_map: &LossMap) -> usize {
        self.visit(&Position::new(0, 0), loss_map);
        while let Some(position) = self.find_next_node() {
            self.visit(&position, loss_map);
        }
        let end = Position::new(self.nrows - 1, self.ncols - 1);
        self.print_trace(&end, loss_map);
        let directions = [Direction::N, Direction::E, Direction::S, Direction::W];
        directions
            .iter()
            .map(|d| self.table[end.dindex(d)].loss)
            .min()
            .unwrap()
    }
}

struct BruteSolver {
    position_vec: Vec<Position>,
    position_set: BTreeSet<Position>,
    directions: Vec<Direction>,
    distances: Vec<usize>,
    loss: usize,
    best_loss: usize,
    best_positions: Vec<Position>,
}

impl BruteSolver {
    fn new() -> BruteSolver {
        let mut position_vec = Vec::new();
        let mut position_set = BTreeSet::new();
        let directions = vec![Direction::N];

        let start = Position::new(0, 0);
        position_vec.push(start);
        position_set.insert(start);

        BruteSolver {
            position_vec,
            position_set,
            directions,
            distances: Vec::new(),
            loss: 0,
            best_loss: usize::MAX,
            best_positions: Vec::new(),
        }
    }

    fn make_move(
        &mut self,
        position: &Position,
        distance: &usize,
        direction: &Direction,
        loss_map: &LossMap,
    ) -> Result<(), ()> {
        let mut current = *position;
        let mut new_positions = Vec::new();
        for _ in 0..*distance {
            current = current.move_by(&1, direction);
            self.loss += loss_map.data[current.index] as usize;
            if self.position_set.contains(&current) {
                return Err(());
            }
            new_positions.push(current);
        }
        for new_position in new_positions {
            self.position_vec.push(new_position);
            self.position_set.insert(new_position);
        }
        self.directions.push(*direction);
        self.distances.push(*distance);
        Ok(())
    }

    fn undo_move(&mut self, loss_map: &LossMap) {
        let distance = self.distances.pop().unwrap();
        self.directions.pop();
        for _ in 0..distance {
            let position = self.position_vec.pop().unwrap();
            self.position_set.remove(&position);
            self.loss -= loss_map.data[position.index] as usize;
        }
    }

    fn complete(&self, loss_map: &LossMap) -> bool {
        let last_position = self.position_vec.last().unwrap();
        *last_position.row() == loss_map.nrows - 1 && *last_position.col() == loss_map.ncols - 1
    }

    fn solve_inner(&mut self, loss_map: &LossMap) {
        let position = *self.position_vec.last().unwrap();
        let last_direction = self.directions.last().unwrap();
        for direction in Direction::all_but(last_direction) {
            for distance in 1..=3 {
                if !position.move_possible(&distance, &direction, loss_map) {
                    continue;
                }
                match self.make_move(&position, &distance, &direction, loss_map) {
                    Ok(()) => {
                        if self.complete(loss_map) {
                            if self.loss < self.best_loss {
                                self.best_loss = self.loss;
                                self.best_positions = self.position_vec.clone();
                            }
                        } else {
                            self.solve_inner(loss_map);
                        }
                        self.undo_move(loss_map);
                    }
                    Err(()) => (),
                }
            }
        }
    }
    fn solve(&mut self, loss_map: &LossMap) -> usize {
        self.solve_inner(loss_map);
        self.best_loss
    }
}

pub fn run(input: &str) -> usize {
    let loss_map = input.parse::<LossMap>().unwrap();
    let fast_loss = {
        let now = Instant::now();
        let mut solver = Solver::from(&loss_map);
        let fast_loss = solver.solve(&loss_map);
        println!("fast took {}", now.elapsed().as_secs_f32());
        fast_loss
    };
    let slow_loss = {
        let now = Instant::now();
        let mut solver = BruteSolver::new();
        let loss = solver.solve(&loss_map);
        println!("slow path:");
        for pos in solver.best_positions {
            println!("  {}", pos);
        }
        println!("slow took {}", now.elapsed().as_secs_f32());
        loss
    };
    assert_eq!(fast_loss, slow_loss);
    fast_loss
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 102);
    }

    #[test]
    fn test2() {
        let input = r#"11
11"#;
        assert_eq!(super::run(input), 2);
    }

    #[test]
    fn test3() {
        let input = r#"111
111
111"#;
        assert_eq!(super::run(input), 4);
    }

    #[test]
    fn test4() {
        let input = r#"1111
1111
1111
1111"#;
        assert_eq!(super::run(input), 6);
    }

    #[test]
    fn test5() {
        let input = r#"11111
11111
11111
11111
11111"#;
        assert_eq!(super::run(input), 8);
    }

    #[test]
    fn test5_b() {
        let input = r#"11111
11111
11111
11111
11121"#;
        assert_eq!(super::run(input), 8);
    }

    #[test]
    fn test5_c() {
        let input = r#"12111
11111
11111
11111
11121"#;
        assert_eq!(super::run(input), 8);
    }

    #[test]
    fn test5_d() {
        let input = r#"11115
55515
51115
51555
51111"#;
        assert_eq!(super::run(input), 12);
    }

    #[test]
    fn test_10() {
        let input = r#"1119999999
9919999999
9119999999
9199111999
9111191999
1119991999
1119111999
1111199999
1199111999
1111111111"#;
        assert_eq!(super::run(input), 20);
    }
}
// 1 1 1 1 .
// 2 . . 1 .
// 1 . . 1 .
// 1 1 1 x 1
// . . . 9 .

// x x x x .
// . . . x x
// . . . . x
// . . . . x
// . . . . x

// x x x x .
// . . . x .
// . . . x x
// . . . . x
// . . . . x
