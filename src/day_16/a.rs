use ndarray::{Array2, Array3};
use std::str::FromStr;

#[derive(Debug, Clone, Copy)]
enum Mirror {
    N,
    V,
    H,
    S,
    B,
}

impl Mirror {
    fn propagate(&self, direction: &Direction) -> Vec<Direction> {
        match self {
            Mirror::N => {
                return vec![direction.straight()];
            }
            Mirror::V => match direction {
                Direction::N | Direction::S => vec![direction.straight()],
                Direction::E | Direction::W => vec![direction.left(), direction.right()],
            },
            Mirror::H => match direction {
                Direction::E | Direction::W => vec![direction.straight()],
                Direction::S | Direction::N => vec![direction.left(), direction.right()],
            },
            Mirror::S => match direction {
                Direction::N | Direction::S => vec![direction.right()],
                Direction::E | Direction::W => vec![direction.left()],
            },
            Mirror::B => match direction {
                Direction::N | Direction::S => vec![direction.left()],
                Direction::E | Direction::W => vec![direction.right()],
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn straight(&self) -> Direction {
        return *self;
    }
    fn left(&self) -> Direction {
        match self {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }
    fn right(&self) -> Direction {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
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

#[derive(Clone, Copy, Debug)]
struct Position {
    row: usize,
    col: usize,
}
impl Position {
    fn new(row: usize, col: usize) -> Self {
        Position { row, col }
    }
    fn move_towards(&self, direction: &Direction, layout: &Layout) -> Option<Self> {
        let (nrows, ncols) = {
            let shape = layout.mirrors.shape();
            (shape[0], shape[1])
        };
        match direction {
            Direction::N => {
                if self.row == 0 {
                    None
                } else {
                    Some(Position::new(self.row - 1, self.col))
                }
            }
            Direction::E => {
                if self.col == ncols - 1 {
                    None
                } else {
                    Some(Position::new(self.row, self.col + 1))
                }
            }
            Direction::S => {
                if self.row == nrows - 1 {
                    None
                } else {
                    Some(Position::new(self.row + 1, self.col))
                }
            }
            Direction::W => {
                if self.col == 0 {
                    None
                } else {
                    Some(Position::new(self.row, self.col - 1))
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Light {
    /// Direction light was moving when it entered position
    direction: Direction,
    position: Position,
}

impl Light {
    fn new(row: usize, col: usize, direction: Direction) -> Light {
        Light {
            direction,
            position: Position { row, col },
        }
    }
    fn row(&self) -> &usize {
        return &self.position.row;
    }
    fn col(&self) -> &usize {
        return &self.position.col;
    }
    fn move_towards(&self, direction: &Direction, layout: &Layout) -> Option<Self> {
        if let Some(new_position) = self.position.move_towards(direction, layout) {
            Some(Light {
                position: new_position,
                direction: *direction,
            })
        } else {
            None
        }
    }
    /// Return a vector of resulting light
    fn propagate(&self, mirror: &Mirror, layout: &Layout) -> Vec<Self> {
        let motions = mirror.propagate(&self.direction);
        motions
            .iter()
            .map(|m| self.move_towards(m, layout))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect::<Vec<_>>()
    }
}

impl TryFrom<char> for Mirror {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Mirror::N),
            '|' => Ok(Mirror::V),
            '-' => Ok(Mirror::H),
            '/' => Ok(Mirror::S),
            '\\' => Ok(Mirror::B),
            _ => Err("Invalid char"),
        }
    }
}

#[derive(Debug)]
struct Layout {
    mirrors: Array2<Mirror>,
    light: Array3<bool>,
}

impl FromStr for Layout {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirrors = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Mirror::try_from(c).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let nrows = mirrors.len();
        let ncols = mirrors[0].len();
        let mirrors = mirrors.concat();
        let mirrors = Array2::from_shape_vec((nrows, ncols), mirrors).unwrap();
        Ok(Layout::from(mirrors))
    }
}

impl From<Array2<Mirror>> for Layout {
    fn from(mirrors: Array2<Mirror>) -> Self {
        let (nrows, ncols) = {
            let shape = mirrors.shape();
            (shape[0], shape[1])
        };
        let ndir = 4;
        let light = Array3::from_elem((nrows, ncols, ndir), false);
        Layout { mirrors, light }
    }
}

impl Layout {
    /// Propagate light according to rules until complete
    fn propagate(&mut self) {
        let mut new_light = vec![Light::new(0, 0, Direction::E)];
        while let Some(light) = new_light.pop() {
            for n in light.propagate(&self.mirrors[[*light.row(), *light.col()]], self) {
                if !self.contains(&n) {
                    self.insert(&n);
                    new_light.push(n);
                }
            }
        }
    }
    fn contains(&self, light: &Light) -> bool {
        todo!()
    }
    fn insert(&self, light: &Light) {
        todo!()
    }
}

pub fn run(input: &str) -> usize {
    let mut layout = input.parse::<Layout>().unwrap();
    layout.propagate();
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 46);
    }
}
