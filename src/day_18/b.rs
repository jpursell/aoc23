use std::{fmt::Display, str::FromStr};

use ndarray::Array2;

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

impl DigInstruction {
    fn new(direction: Direction, distance: usize) -> DigInstruction {
        DigInstruction {
            direction,
            distance,
        }
    }
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

impl DigPlan {
    fn new(instructions: Vec<DigInstruction>) -> DigPlan {
        DigPlan { instructions }
    }
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

    fn index_from(&self, position: &Position) -> [usize; 2] {
        [
            (self.row() - position.row()) as usize,
            (self.col() - position.col()) as usize,
        ]
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

    fn max(&self, other: &Position) -> Position {
        Position::new(self.row().max(other.row()), self.col().max(other.col()))
    }

    fn min(&self, other: &Position) -> Position {
        Position::new(self.row().min(other.row()), self.col().min(other.col()))
    }
}

#[derive(Debug)]
struct PolyLagoon {
    vertices: Vec<Position>,
}

enum ElementType {
    Empty,
    UpperLeft,
    UpperRight,
    BottomLeft,
    BottomRight,
    Virtical,
    Horizontal,
}

// R 1
// D 1
// L 1
// U 1
//
// 0 1
// 3 2

impl PolyLagoon {
    // fn check_area(dug: &Array2<bool>, irow: usize, icol: usize) -> ElementType {
    //     if !dug[[irow, icol]] {
    //         return ElementType::Empty;
    //     }
    //     let left = dug[[irow, icol - 1]];
    //     let right = dug[[irow, icol + 1]];
    //     let top = dug[[irow - 1, icol]];
    //     let bottom = dug[[irow + 1, icol]];

    //     if top && bottom {
    //         return ElementType::Virtical;
    //     }
    //     if left && right {
    //         return ElementType::Horizontal;
    //     }
    //     if top {
    //         if right {
    //             return ElementType::BottomLeft;
    //         }
    //         if left {
    //             return ElementType::BottomRight;
    //         }
    //     }
    //     if bottom {
    //         if right {
    //             return ElementType::UpperLeft;
    //         }
    //         if left {
    //             return ElementType::UpperRight;
    //         }
    //     }
    //     panic!()
    // }
    fn new(dig_plan: &DigPlan) -> PolyLagoon {
        // find boundaries
        let mut position = Position::new(0, 0);
        let mut vertices = Vec::new();
        dig_plan.instructions.iter().for_each(|instruction| {
            position = position.move_by(&instruction.direction, &instruction.distance);
            dbg!(&position);
            match instruction.direction {
                Direction::L => {
                    vertices.push(Position::new(position.row(), position.col()));
                }
                Direction::U => {
                    vertices.push(Position::new(position.row() - 1, position.col()));
                }
                Direction::D => {
                    vertices.push(Position::new(position.row(), position.col() + 1));
                }
                Direction::R => {
                    vertices.push(Position::new(position.row() - 1, position.col() + 1));
                }
            }
            dbg!(&vertices);
        });

        // init data
        // let nrows = usize::try_from(max_position.row() - min_position.row() + 1).unwrap();
        // let ncols = usize::try_from(max_position.col() - min_position.col() + 1).unwrap();
        // let mut dug = Array2::from_elem((nrows, ncols), false);

        // dig trench
        // {
        //     let mut position = Position::new(0, 0);
        //     dig_plan.instructions.iter().for_each(|instruction| {
        //         (1..=instruction.distance).for_each(|_| {
        //             position = position.move_by(&instruction.direction, &1);
        //             dug[position.index_from(&min_position)] = true;
        //         })
        //     });
        // }

        // let trench = dug.clone();

        // fill trench
        // for irow in 1..(nrows - 1) {
        //     let mut inside = false;
        //     let mut last = None;
        //     for icol in 1..(ncols - 1) {
        //         match Lagoon::check_area(&trench, irow, icol) {
        //             ElementType::BottomLeft => {
        //                 // println!("{} {} BL", irow, icol);
        //                 assert!(last.is_none());
        //                 last = Some(ElementType::BottomLeft);
        //             }
        //             ElementType::Empty => (),
        //             ElementType::UpperLeft => {
        //                 // println!("{} {} UL", irow, icol);
        //                 assert!(last.is_none());
        //                 last = Some(ElementType::UpperLeft)
        //             }
        //             ElementType::UpperRight => {
        //                 // println!("{} {} UR", irow, icol);
        //                 match last {
        //                     Some(ElementType::BottomLeft) => {
        //                         inside = !inside;
        //                         last = None;
        //                     }
        //                     Some(ElementType::UpperLeft) => {
        //                         last = None;
        //                     }
        //                     _ => panic!(),
        //                 }
        //             }
        //             ElementType::BottomRight => {
        //                 // println!("{} {} BR", irow, icol);
        //                 match last {
        //                     Some(ElementType::BottomLeft) => {
        //                         last = None;
        //                     }
        //                     Some(ElementType::UpperLeft) => {
        //                         inside = !inside;
        //                         last = None;
        //                     }
        //                     _ => panic!(),
        //                 }
        //             }
        //             ElementType::Virtical => {
        //                 // println!("{} {} V", irow, icol);
        //                 inside = !inside;
        //             }
        //             ElementType::Horizontal => (),
        //         }
        //         if inside {
        //             dug[[irow, icol]] = true;
        //         }
        //     }
        // }

        PolyLagoon { vertices }
    }

    // fn count(&self) -> usize {
    //     self.dug.iter().filter(|e| **e).count()
    // }
}

impl Display for PolyLagoon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.vertices
            .iter()
            .for_each(|x| writeln!(f, "{}", x).unwrap());
        Ok(())
    }
}

pub fn run(input: &str) -> usize {
    let plan = input.parse::<DigPlan>().unwrap();
    let lagoon = PolyLagoon::new(&plan);
    println!("{}", lagoon);
    // println!("{}", lagoon);
    // lagoon.count()
    0
}

#[cfg(test)]
mod tests {
    use super::{DigInstruction, DigPlan, PolyLagoon};

    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 62);
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
        println!("{}", lagoon);
    }
}
