use std::{collections::BTreeSet, str::FromStr};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn run(input: &str) -> u64 {
    input.parse::<Maze>().unwrap().count_inside()
}

struct Maze {
    nrows: usize,
    ncols: usize,
    map: Vec<Vec<char>>,
}

impl FromStr for Maze {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        assert!(map.iter().all(|line| line.len() == map[0].len()));
        Ok(Maze {
            nrows: map.len(),
            ncols: map[0].len(),
            map,
        })
    }
}

#[derive(Ord, PartialOrd, Eq, Clone, Copy, Debug, EnumIter, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

fn opposite_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::N => Direction::S,
        Direction::E => Direction::W,
        Direction::S => Direction::N,
        Direction::W => Direction::E,
    }
}

fn get_directions(c: &char) -> Option<(Direction, Direction)> {
    match c {
        '|' => Some((Direction::N, Direction::S)),
        '-' => Some((Direction::E, Direction::W)),
        'L' => Some((Direction::N, Direction::E)),
        'J' => Some((Direction::N, Direction::W)),
        '7' => Some((Direction::S, Direction::W)),
        'F' => Some((Direction::S, Direction::E)),
        '.' => None,
        _ => panic!("got unexpect val {}", c),
    }
}

fn tile_supports_dir(tile: &char, direction: &Direction) -> bool {
    let Some((d1, d2)) = get_directions(tile) else {
        return false;
    };
    if d1 == *direction || d2 == *direction {
        return true;
    }
    false
}

fn can_go_to_tile(tile: &char, direction: &Direction) -> bool {
    if *tile == 'S' {
        return true;
    }
    tile_supports_dir(tile, &opposite_direction(direction))
}

struct MazeWalker<'a> {
    maze: &'a Maze,
    pos: (usize, usize),
    last_direction: Option<Direction>,
}

impl<'a> MazeWalker<'a> {
    fn get_tile(&self, direction: &Direction) -> Option<char> {
        match direction {
            Direction::N => {
                if self.pos.0 == 0 {
                    return None;
                }
                return Some(self.maze.map[self.pos.0 - 1][self.pos.1]);
            }
            Direction::E => {
                if self.pos.1 == self.maze.ncols - 1 {
                    return None;
                }
                return Some(self.maze.map[self.pos.0][self.pos.1 + 1]);
            }
            Direction::S => {
                if self.pos.0 == self.maze.nrows - 1 {
                    return None;
                }
                return Some(self.maze.map[self.pos.0 + 1][self.pos.1]);
            }
            Direction::W => {
                if self.pos.1 == 0 {
                    return None;
                }
                return Some(self.maze.map[self.pos.0][self.pos.1 - 1]);
            }
        }
    }

    fn can_move(&self, direction: &Direction) -> bool {
        let Some(tile) = self.get_tile(direction) else {
            return false;
        };
        can_go_to_tile(&tile, direction)
    }

    fn new(maze: &Maze) -> MazeWalker {
        MazeWalker {
            maze,
            pos: maze.find_start().expect("No start!"),
            last_direction: None,
        }
    }

    fn advance_position(&mut self, direction: &Direction) {
        self.pos = match direction {
            Direction::N => (self.pos.0 - 1, self.pos.1),
            Direction::E => (self.pos.0, self.pos.1 + 1),
            Direction::S => (self.pos.0 + 1, self.pos.1),
            Direction::W => (self.pos.0, self.pos.1 - 1),
        };
        self.last_direction = Some(*direction);
    }

    fn sub_start(&self) -> char {
        assert_eq!(self.last_direction, None);
        let dirs = Direction::iter()
            .filter(|d| self.can_move(d))
            .collect::<BTreeSet<Direction>>();
        assert_eq!(2, dirs.len());
        if dirs.contains(&Direction::N) {
            if dirs.contains(&Direction::E) {
                return 'L';
            } else if dirs.contains(&Direction::W) {
                return 'J';
            } else if dirs.contains(&Direction::S) {
                return '|';
            } else {
                panic!();
            }
        } else if dirs.contains(&Direction::S) {
            if dirs.contains(&Direction::E) {
                return 'F';
            } else if dirs.contains(&Direction::W) {
                return '7';
            } else {
                panic!();
            }
        } else if dirs.contains(&Direction::E) && dirs.contains(&Direction::W) {
            return '-';
        }
        panic!();
    }
    fn make_move(&mut self) -> Option<()> {
        match self.last_direction {
            Some(ld) => {
                let tile = &self.maze.map[self.pos.0][self.pos.1];
                let directions = get_directions(tile).expect("Not on a valid tile!");
                let back = opposite_direction(&ld);
                let direction = if directions.0 == back {
                    directions.1
                } else if directions.1 == back {
                    directions.0
                } else {
                    panic!(
                        "directions {:?} from tile {:?} did not match back {:?}",
                        directions, tile, back
                    )
                };
                assert!(self.can_move(&direction));
                self.advance_position(&direction);
                if self.maze.map[self.pos.0][self.pos.1] == 'S' {
                    return None;
                }
            }
            None => {
                for dir in Direction::iter() {
                    if self.can_move(&dir) {
                        self.advance_position(&dir);
                        break;
                    }
                }
            }
        }
        Some(())
    }
}

impl Maze {
    fn find_start(&self) -> Result<(usize, usize), &'static str> {
        for (irow, row) in self.map.iter().enumerate() {
            for (icol, col) in row.iter().enumerate() {
                if *col == 'S' {
                    return Ok((irow, icol));
                }
            }
        }
        Err("No start")
    }

    fn sub_start(&self) -> char {
        MazeWalker::new(self).sub_start()
    }

    fn count_inside(&self) -> u64 {
        let mut walker = MazeWalker::new(self);
        let mut path = BTreeSet::new();
        path.insert(walker.pos);
        loop {
            match walker.make_move() {
                Some(_) => (),
                None => {
                    break;
                }
            }
            path.insert(walker.pos);
        }
        path.insert(walker.pos);
        let mut count = 0;
        for irow in 0..self.nrows {
            let mut crossings = 0;
            let mut on_ridge = false;
            let mut ridge_start = None;
            for icol in 0..self.ncols {
                let pos = (irow, icol);
                let tile = self.map[pos.0][pos.1];
                let tile = if tile == 'S' { self.sub_start() } else { tile };
                if path.contains(&pos) {
                    match tile {
                        '|' => {
                            crossings += 1;
                        }
                        'F' => {
                            if on_ridge {
                                panic!();
                            } else {
                                on_ridge = true;
                                ridge_start = Some(tile);
                            }
                        }
                        'J' => {
                            if on_ridge {
                                match ridge_start {
                                    Some('F') => {
                                        crossings += 1;
                                        on_ridge = false;
                                    }
                                    Some('L') => {
                                        on_ridge = false;
                                    }
                                    _ => panic!(),
                                }
                            } else {
                                panic!();
                            }
                        }
                        '7' => {
                            if on_ridge {
                                match ridge_start {
                                    Some('L') => {
                                        crossings += 1;
                                        on_ridge = false;
                                    }
                                    Some('F') => {
                                        on_ridge = false;
                                    }
                                    _ => panic!(),
                                }
                            } else {
                                panic!();
                            }
                        }
                        'L' => {
                            if on_ridge {
                                panic!();
                            } else {
                                on_ridge = true;
                                ridge_start = Some(tile);
                            }
                        }
                        '-' => {
                            assert!(on_ridge);
                        }
                        _ => (),
                    }
                } else {
                    if crossings % 2 == 1 {
                        count += 1;
                    }
                }
            }
            assert!(!on_ridge);
        }
        count
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test2() {
        let input = include_str!("example_data_2.txt");
        assert_eq!(super::run(input), 4);
    }
    #[test]
    fn test3() {
        let input = include_str!("example_data_3.txt");
        assert_eq!(super::run(input), 8);
    }
    #[test]
    fn test4() {
        let input = include_str!("example_data_4.txt");
        assert_eq!(super::run(input), 10);
    }
}
