use std::{fmt::Display, str::FromStr, time::Instant};

use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Position {
    vec: [i64; 3],
}
impl Position {
    fn new(vec: [i64; 3]) -> Position {
        Position { vec }
    }
    fn sum(&self) -> i64 {
        self.vec.iter().sum()
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
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(vec.len(), 3);
        let vec = [vec[0], vec[1], vec[2]];
        Ok(Position::new(vec))
    }
}
#[derive(Debug, Clone, Copy)]
struct Velocity {
    vec: [i64; 3],
}
impl Velocity {
    fn new(vec: [i64; 3]) -> Velocity {
        // vec.iter().for_each(|x| {
        //     assert_ne!(*x, 0);
        // });
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
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        assert_eq!(vec.len(), 3);
        let vec = [vec[0], vec[1], vec[2]];
        Ok(Velocity::new(vec))
    }
}
#[derive(Clone, Copy, Debug)]
struct InitialCondition {
    position: Position,
    velocity: Velocity,
}
impl InitialCondition {
    fn new(position: Position, velocity: Velocity) -> InitialCondition {
        InitialCondition { position, velocity }
    }
    fn position_sum(&self) -> i64 {
        self.position.sum()
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
/// Return rock InitialConfition that will intersect both stones at given times
fn solve_rock(t0: usize, stone0: &InitialCondition, vel: &Velocity) -> InitialCondition {
    let p0 = stone0
        .position
        .vec
        .iter()
        .zip(stone0.velocity.vec.iter())
        .map(|(&p, &v)| p + v * t0 as i64)
        .collect::<Vec<_>>();
    let pos = p0
        .iter()
        .zip(vel.vec.iter())
        .map(|(&p, &v)| p - v * t0 as i64)
        .collect::<Vec<_>>();
    let pos = Position::new([pos[0], pos[1], pos[2]]);
    InitialCondition::new(pos, vel.clone())
}
struct HailCloud {
    stones: Vec<InitialCondition>,
}
impl HailCloud {
    fn new(stones: Vec<InitialCondition>) -> HailCloud {
        HailCloud { stones }
    }

    /// Solve for rock that will pass through all hail positions
    /// and return sum of initial position coords
    fn run(&self, maxv:i64) -> i64 {
        let start = Instant::now();
        let mut last = start;
        for vx in -maxv..maxv {
            if last.elapsed().as_secs() > 5 {
                println!("{} vx: {}", start.elapsed().as_secs_f32(), vx);
                last = Instant::now();
            }
            if let Some(rock) = self.check_vx(vx, maxv) {
                println!("rock: {}", rock);
                return rock.position_sum();
            }
        }
        panic!("no solution found");
    }
    fn check_vx(&self, vx: i64, maxv:i64) -> Option<InitialCondition> {
        let rock = (-maxv..maxv)
            .into_par_iter()
            .map(|vy| self.check_vy(vx, vy, maxv))
            .filter(|x| x.is_some())
            .collect::<Vec<_>>();
        match rock.len() {
            0 => None,
            1 => rock[0],
            _ => panic!(),
        }
    }
    fn check_vy(&self, vx: i64, vy: i64, maxv:i64) -> Option<InitialCondition> {
        for vz in -maxv..maxv {
            for i in 0..self.stones.len() {
                // assume hits at t: 1
                let v = Velocity::new([vx, vy, vz]);
                let rock = solve_rock(1, &self.stones[i], &v);
                if self.verify_rock(i, &rock) {
                    return Some(rock);
                }
            }
        }
        None
    }
    /// Return true if rock intersects all stones. Assume stones at i and j are already checked
    fn verify_rock(&self, i: usize, rock: &InitialCondition) -> bool {
        for k in 0..self.stones.len() {
            if k == i {
                continue;
            }
            // stone_loc = stone_pos + stone_vel * t
            // rock_loc = rock_pos + rock_vel * t
            // stone_pos + stone_vel * t = rock_pos + rock_vel * t
            // stone_pos - rock_pos = t * (rock_vel - stone_vel)
            // t = (stone_pos - rock_pos) / (rock_vel - stone_vel)
            let stone = &self.stones[k];
            if rock.velocity.vec[0] as i64 - stone.velocity.vec[0] as i64 == 0 {
                return false;
            }
            let t = (stone.position.vec[0] as i64 - rock.position.vec[0] as i64)
                / (rock.velocity.vec[0] as i64 - stone.velocity.vec[0] as i64);
            if t < 0 {
                return false;
            }
            for n in 0..3 {
                if rock.position.vec[n] as i64 + rock.velocity.vec[n] as i64 * t
                    != stone.position.vec[n] as i64 + stone.velocity.vec[n] as i64 * t
                {
                    return false;
                }
            }
        }
        true
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
pub fn run(input: &str, maxv:i64) -> i64 {
    let hail = input.parse::<HailCloud>().unwrap();
    println!("{}", hail);
    hail.run(maxv)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 10), 47);
    }
}
