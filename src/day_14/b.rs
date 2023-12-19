use ndarray::{Array2, ArrayViewMut1, Axis};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

#[derive(PartialEq, Debug)]
struct RockField {
    rocks: Array2<Rock>,
    nrows: usize,
    // ncols: usize,
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
        let nrows = rocks.len();
        let ncols = rocks[0].len();
        let rocks = rocks.concat();
        let rocks = Array2::from_shape_vec((nrows, ncols), rocks).unwrap();
        Ok(RockField {
            rocks,
            nrows,
            // ncols,
        })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl RockField {
    fn roll_arr(arr: &mut ArrayViewMut1<Rock>, forward: bool) {
        if forward {
            // if forward then move rocks to higher index
            let mut write = arr.len() - 1;
            for read in (0..arr.len()).rev() {
                if arr[read] == Rock::S {
                    write = read;
                }
                while write > 0 && arr[write] != Rock::N {
                    write -= 1;
                }
                if read >= write || arr[write] != Rock::N {
                    continue;
                }
                if arr[read] == Rock::R {
                    arr[read] = Rock::N;
                    arr[write] = Rock::R;
                }
            }
        } else {
            // if !forward then move rocks to lower index
            let mut write = 0;
            for read in 0..arr.len() {
                if arr[read] == Rock::S {
                    write = read;
                }
                while write < arr.len() - 1 && arr[write] != Rock::N {
                    write += 1;
                }
                if read <= write || arr[write] != Rock::N {
                    continue;
                }
                if arr[read] == Rock::R {
                    arr[read] = Rock::N;
                    arr[write] = Rock::R;
                }
            }
        }
    }
    fn roll(&mut self, direction: Direction) {
        match direction {
            Direction::S => {
                self.rocks
                    .axis_iter_mut(Axis(1))
                    .for_each(|mut c| RockField::roll_arr(&mut c, true));
            }
            Direction::N => {
                self.rocks
                    .axis_iter_mut(Axis(1))
                    .for_each(|mut c| RockField::roll_arr(&mut c, false));
            }
            Direction::E => {
                self.rocks
                    .axis_iter_mut(Axis(0))
                    .for_each(|mut c| RockField::roll_arr(&mut c, true));
            }
            Direction::W => {
                self.rocks
                    .axis_iter_mut(Axis(0))
                    .for_each(|mut c| RockField::roll_arr(&mut c, false));
            }
        }
    }
    fn count_rocks(&self) -> usize {
        self.rocks
            .indexed_iter()
            .map(
                |((irow, _), r)| {
                    if *r == Rock::R {
                        self.nrows - irow
                    } else {
                        0
                    }
                },
            )
            .sum::<usize>()
    }
}
pub fn run(input: &str) -> usize {
    let mut field = input.parse::<RockField>().unwrap();
    let directions = vec![Direction::N, Direction::W, Direction::S, Direction::E];
    // let ncycles = 1_000_000_000;
    for cycle in 0..100 {
        for direction in &directions {
            field.roll(*direction);
        }
        let n = field.count_rocks();
        if n == 64 {
            println!("{} {} <=====", cycle, field.count_rocks());
        } else {
            println!("{} {}", cycle, field.count_rocks());
        }
    }
    field.count_rocks()
}

#[cfg(test)]
mod tests {
    use super::{Direction, RockField};

    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 64);
    }
    #[test]
    fn roll_test() {
        let input = include_str!("example_data.txt");
        let mut field = input.parse::<RockField>().unwrap();
        field.roll(Direction::N);
        let expected = include_str!("example_data_n.txt");
        let expected = expected.parse::<RockField>().unwrap();
        assert_eq!(field, expected);
    }
    #[test]
    fn test_a() {
        let input = include_str!("example_data.txt");
        let mut field = input.parse::<RockField>().unwrap();
        field.roll(Direction::N);
        assert_eq!(field.count_rocks(), 136);
    }
}
