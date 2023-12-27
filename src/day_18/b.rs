use std::{fmt::Display, str::FromStr};

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::U => write!(f, "U"),
            Direction::D => write!(f, "D"),
            Direction::L => write!(f, "L"),
            Direction::R => write!(f, "R"),
        }
    }
}

impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(Direction::R),
            "1" => Ok(Direction::D),
            "2" => Ok(Direction::L),
            "3" => Ok(Direction::U),
            _ => Err("Unknown direction"),
        }
    }
}

#[derive(Debug)]
struct DigInstruction {
    direction: Direction,
    distance: usize,
}

impl Display for DigInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.direction, self.distance)
    }
}

impl FromStr for DigInstruction {
    type Err = &'static str;
    /// Read in something like "R 10 (#ffffff)"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut s = s.split_whitespace().skip(2).next().unwrap();
        s = &s[2..8];
        let (distance, direction) = s.split_at(5);
        let distance = usize::from_str_radix(distance, 16).unwrap();
        let direction = direction.parse::<Direction>().unwrap();
        Ok(DigInstruction {
            direction,
            distance,
        })
    }
}

#[derive(Debug)]
struct DigPlan {
    instructions: Vec<DigInstruction>,
}

impl Display for DigPlan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.instructions
            .iter()
            .for_each(|x| writeln!(f, "{}", x).unwrap());
        Ok(())
    }
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

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.index[0], self.index[1])
    }
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

    fn move_by(&self, direction: &Direction, distance: &usize) -> Position {
        let distance = *distance as i64;
        match direction {
            Direction::D => Position::new(self.row() + distance, self.col()),
            Direction::U => Position::new(self.row() - distance, self.col()),
            Direction::L => Position::new(self.row(), self.col() - distance),
            Direction::R => Position::new(self.row(), self.col() + distance),
        }
    }
}

#[derive(Debug)]
struct PolyLagoon {
    segments: Vec<Segment>,
}

#[derive(Debug)]
struct Segment {
    points: [Position; 2],
}

impl Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.points[0], self.points[1])
    }
}

impl Segment {
    fn new(a: Position, b: Position) -> Segment {
        let points = [a, b];
        Segment { points }
    }
    fn rows(&self) -> [i64; 2] {
        [self.points[0].row(), self.points[1].row()]
    }
    fn cols(&self) -> [i64; 2] {
        [self.points[0].col(), self.points[1].col()]
    }
    fn vertical(&self) -> bool {
        let rows = self.rows();
        let out = rows[0] != rows[1];
        if out {
            let cols = self.cols();
            assert_eq!(cols[0], cols[1]);
        }
        out
    }
    fn contains(&self, row: i64) -> bool {
        let rows = {
            let mut rows = self.rows();
            if rows[1] < rows[0] {
                rows.swap(1, 0);
            }
            rows
        };
        if row < rows[0] || row >= rows[1] {
            return false;
        }
        true
    }
}

impl PolyLagoon {
    fn new(dig_plan: &DigPlan) -> PolyLagoon {
        // create vertices
        let mut position = Position::new(0, 0);
        let mut vertices = Vec::new();
        dig_plan
            .instructions
            .iter()
            .enumerate()
            .for_each(|(i, instruction)| {
                let next_instruction = match dig_plan.instructions.get(i + 1) {
                    Some(ni) => ni,
                    None => dig_plan.instructions.first().unwrap(),
                };
                position = position.move_by(&instruction.direction, &instruction.distance);
                match instruction.direction {
                    Direction::L => match next_instruction.direction {
                        Direction::U => {
                            vertices.push(Position::new(position.row() + 1, position.col()));
                        }
                        Direction::D => {
                            vertices.push(Position::new(position.row() + 1, position.col() + 1));
                        }
                        _ => panic!(),
                    },
                    Direction::U => match next_instruction.direction {
                        Direction::L => {
                            vertices.push(Position::new(position.row() + 1, position.col()));
                        }
                        Direction::R => {
                            vertices.push(Position::new(position.row(), position.col()));
                        }
                        _ => panic!(),
                    },
                    Direction::D => match next_instruction.direction {
                        Direction::L => {
                            vertices.push(Position::new(position.row() + 1, position.col() + 1));
                        }
                        Direction::R => {
                            vertices.push(Position::new(position.row(), position.col() + 1));
                        }
                        _ => panic!(),
                    },
                    Direction::R => match next_instruction.direction {
                        Direction::U => {
                            vertices.push(Position::new(position.row(), position.col()));
                        }
                        Direction::D => {
                            vertices.push(Position::new(position.row(), position.col() + 1));
                        }
                        _ => panic!(),
                    },
                }
            });

        // vertices.iter().for_each(|v| println!("{}", v));

        let mut segments = vertices
            .windows(2)
            .map(|positions| Segment::new(positions[0], positions[1]))
            .collect::<Vec<_>>();
        segments.push(Segment::new(
            *vertices.first().unwrap(),
            *vertices.last().unwrap(),
        ));
        let segments = segments
            .into_iter()
            .filter(|s| s.vertical())
            .collect::<Vec<_>>();

        PolyLagoon { segments }
    }

    fn sorted_rows(&self) -> Vec<i64> {
        let mut rows = self
            .segments
            .iter()
            .map(|s| s.rows())
            .collect::<Vec<_>>()
            .concat();
        rows.sort();
        rows
    }

    fn count(&self) -> usize {
        let rows = self.sorted_rows();
        let mut area = 0;
        for row_slice in rows.windows(2) {
            if row_slice[0] == row_slice[1] {
                continue;
            }
            let cols = {
                let mut cols = self
                    .segments
                    .iter()
                    .filter(|s| s.contains(row_slice[0]))
                    .map(|s| s.cols()[0])
                    .collect::<Vec<_>>();
                cols.sort();
                cols
            };
            assert_eq!(cols.len() % 2, 0);
            let width = cols
                .chunks(2)
                .map(|c| c[1] - c[0])
                .inspect(|w| assert!(*w > 0))
                .map(|w| w as usize)
                .sum::<usize>();
            let length = row_slice[1] - row_slice[0];
            assert!(length > 0);
            area += width * length as usize;
        }
        area
    }
}

impl Display for PolyLagoon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.segments
            .iter()
            .for_each(|x| writeln!(f, "{}", x).unwrap());
        Ok(())
    }
}

pub fn run(input: &str) -> usize {
    let plan = input.parse::<DigPlan>().unwrap();
    let lagoon = PolyLagoon::new(&plan);
    // println!("{}", lagoon);
    lagoon.count()
}

#[cfg(test)]
mod tests {
    use super::{DigInstruction, DigPlan, Direction, PolyLagoon};

    impl DigInstruction {
        fn new(direction: Direction, distance: usize) -> DigInstruction {
            DigInstruction {
                direction,
                distance,
            }
        }
    }

    impl DigPlan {
        fn new(instructions: Vec<DigInstruction>) -> DigPlan {
            DigPlan { instructions }
        }
    }

    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 952_408_144_115);
    }
    #[test]
    fn test_small() {
        let plan = DigPlan::new(vec![
            DigInstruction::new(super::Direction::R, 1),
            DigInstruction::new(super::Direction::D, 1),
            DigInstruction::new(super::Direction::L, 1),
            DigInstruction::new(super::Direction::U, 1),
        ]);
        let lagoon = PolyLagoon::new(&plan);
        // println!("{}", lagoon);
        assert_eq!(lagoon.count(), 4);
    }

    #[test]
    fn test_four_square() {
        //   01234567
        // 0 ###.###
        // 1 #O###O#
        // 2 ##OOO##
        // 3 .#OOO#.
        // 4 ##OOO##
        // 5 #O###O#
        // 6 ###.###
        // 7
        let plan = DigPlan::new(vec![
            DigInstruction::new(super::Direction::R, 2),
            DigInstruction::new(super::Direction::D, 1),
            DigInstruction::new(super::Direction::R, 2),
            DigInstruction::new(super::Direction::U, 1),
            DigInstruction::new(super::Direction::R, 2),
            DigInstruction::new(super::Direction::D, 2),
            DigInstruction::new(super::Direction::L, 1),
            DigInstruction::new(super::Direction::D, 2),
            DigInstruction::new(super::Direction::R, 1),
            DigInstruction::new(super::Direction::D, 2),
            DigInstruction::new(super::Direction::L, 2),
            DigInstruction::new(super::Direction::U, 1),
            DigInstruction::new(super::Direction::L, 2),
            DigInstruction::new(super::Direction::D, 1),
            DigInstruction::new(super::Direction::L, 2),
            DigInstruction::new(super::Direction::U, 2),
            DigInstruction::new(super::Direction::R, 1),
            DigInstruction::new(super::Direction::U, 2),
            DigInstruction::new(super::Direction::L, 1),
            DigInstruction::new(super::Direction::U, 2),
        ]);
        let lagoon = PolyLagoon::new(&plan);
        // println!("{}", lagoon);
        assert_eq!(lagoon.count(), 45);
    }
}
