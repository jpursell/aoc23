use ndarray::{Array2, Array3};
use std::{collections::BTreeSet, str::FromStr};

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
            Mirror::V => todo!(),
            Mirror::H => todo!(),
            Mirror::S => todo!(),
            Mirror::B => todo!(),
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

#[derive(Clone, Copy, Debug)]
struct Position {
    row: usize,
    col: usize,
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
    /// Return a vector of resulting light
    fn propagate(&self, mirror: &Mirror) -> Vec<Self> {
        let motions = mirror.propagate(self.direction);
        // break out into a mirror.propagate that returns a vector of movements
        // use/create position.move(direction) -> position
        todo!()
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
            let p = light.propagate(self.mirrors[[light.row(), light.col()]]);
            todo!()
        }
    }
}

pub fn run(input: &str) -> usize {
    let layout = input.parse::<Layout>().unwrap();
    dbg!(&layout.new_light);
    dbg!(&layout.light);
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
