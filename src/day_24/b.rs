use std::{fmt::Display, str::FromStr};

#[derive(Clone, Copy)]
struct Position {
    vec: [usize; 3],
}
impl Position {
    fn new(vec: [usize; 3]) -> Position {
        Position { vec }
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.vec[0], self.vec[1], self.vec[2])
    }
}
impl FromStr for Position {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s
            .split(", ")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(vec.len(), 3);
        let vec = [vec[0], vec[1], vec[2]];
        Ok(Position::new(vec))
    }
}
#[derive(Clone, Copy)]
struct Velocity {
    vec: [i32; 3],
}
impl Velocity {
    fn new(vec: [i32; 3]) -> Velocity {
        vec.iter().for_each(|x| {
            assert_ne!(*x, 0);
        });
        Velocity { vec }
    }
}
impl Display for Velocity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.vec[0], self.vec[1], self.vec[2])
    }
}
impl FromStr for Velocity {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec = s
            .split(", ")
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(vec.len(), 3);
        let vec = [vec[0], vec[1], vec[2]];
        Ok(Velocity::new(vec))
    }
}
#[derive(Clone, Copy)]
struct InitialCondition {
    position: Position,
    velocity: Velocity,
}
impl InitialCondition {
    fn new(position: Position, velocity: Velocity) -> InitialCondition {
        InitialCondition { position, velocity }
    }
}
impl FromStr for InitialCondition {
    type Err = ();
    /// Parse things like 19, 13, 30 @ -2,  1, -2
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once(" @ ").unwrap();
        let pos = pos.parse::<Position>().unwrap();
        let vel = vel.parse::<Velocity>().unwrap();
        Ok(InitialCondition::new(pos, vel))
    }
}
impl Display for InitialCondition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {}", self.position, self.velocity)
    }
}
struct HailCloud {
    stones: Vec<InitialCondition>,
}
impl HailCloud {
    fn new(stones: Vec<InitialCondition>) -> HailCloud {
        HailCloud {
            stones,
        }
    }
}
impl FromStr for HailCloud {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones = s
            .lines()
            .map(|line| line.parse::<InitialCondition>().unwrap())
            .collect::<Vec<_>>();
        Ok(HailCloud::new(stones))
    }
}
impl Display for HailCloud {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stone in &self.stones {
            writeln!(f, "{}", stone)?
        }
        Ok(())
    }
}
pub fn run(input: &str) -> usize {
    let hail = input.parse::<HailCloud>().unwrap();
    println!("{}", hail);
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 47);
    }
}
