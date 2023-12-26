use std::str::FromStr;

use ndarray::Array2;

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => Err("Unknown direction"),
        }
    }
}

#[derive(Debug)]
struct DigInstruction {
    direction: Direction,
    distance: usize,
    color: String,
}

impl FromStr for DigInstruction {
    type Err = &'static str;
    /// Read in something like "R 10 (#ffffff)"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace();
        let direction = s.next().unwrap().parse::<Direction>().unwrap();
        let distance = s.next().unwrap().parse::<usize>().unwrap();
        let color = s.next().unwrap().to_string();
        Ok(DigInstruction {
            direction,
            distance,
            color,
        })
    }
}

#[derive(Debug)]
struct DigPlan {
    instructions: Vec<DigInstruction>,
}

impl FromStr for DigPlan {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DigPlan {
            instructions: s
                .lines()
                .map(|line| line.parse::<DigInstruction>().unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    index: [i64; 2],
}

impl Position {
    fn new(row: i64, col: i64) -> Position {
        Position { index: [row, col] }
    }

    fn row(&self) -> usize {
        // todo fix all the position code to allow for negative
        self.index[0]
    }

    fn col(&self) -> usize {
        self.index[1]
    }

    fn move_by(&self, direction: &Direction, distance: &usize) -> Position {
        match direction {
            Direction::D => Position::new(self.row() + distance, self.col()),
            Direction::U => Position::new(self.row() - distance, self.col()),
            Direction::L => Position::new(self.row(), self.col() - distance),
            Direction::R => Position::new(self.row(), self.col() + distance),
        }
    }

    fn max(&self, other: &Position) -> Position {
        Position::new(self.row().max(other.row()), self.col().max(other.col()))
    }

    fn min(&self, other: &Position) -> Position {
        Position::new(self.row().min(other.row()), self.col().min(other.col()))
    }
}

struct Lagoon {
    dug: Array2<bool>,
    nrows: usize,
    ncols: usize,
    offset: Position,
}

impl Lagoon {
    fn new(dig_plan: &DigPlan) -> Lagoon {
        // find boundaries
        let mut position = Position::new(0, 0);
        let mut min_position = position;
        let mut max_position = position;
        dig_plan.instructions.iter().for_each(|instruction| {
            position = position.move_by(&instruction.direction, &instruction.distance);
            min_position = min_position.min(&position);
            max_position = max_position.max(&position);
        });

        // init data
        let nrows = max_position.row() - min_position.row() + 1;
        let ncols = max_position.col() - min_position.col() + 1;
        let mut dug = Array2::from_elem((nrows, ncols), false);

        // todo: dig trench
        let mut position = Position::new(0, 0);
        dig_plan.instructions.iter().for_each(|instruction| {
            dug[position.index] = true;
            (1..=instruction.distance).for_each(|_| {
                position = position.move_by(&instruction.direction, &instruction.distance);
            })
        });
        Lagoon {
            dug,
            nrows,
            ncols,
            offset: min_position,
        }
    }
}

pub fn run(input: &str) -> usize {
    let plan = input.parse::<DigPlan>().unwrap();
    dbg!(&plan);
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 0);
    }
}
