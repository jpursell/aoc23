use std::{fmt::Display, str::FromStr, time::Instant};

use itertools::Itertools;

use ndarray::{s, Array1, Array2, Axis};
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
    fn run(&self) -> i64 {
        let start = Instant::now();
        let mut last = start;
        let mut maxv = 0;
        loop {
            maxv += 1;
            if last.elapsed().as_secs() > 5 {
                println!("{} maxv: {}", start.elapsed().as_secs_f32(), maxv);
                last = Instant::now();
            }
            if let Some(rock) = self.check_maxv(maxv) {
                println!("rock: {}", rock);
                return rock.position_sum();
            }
        }
    }
    /// Make a guess at the rock's initial conditions by observing
    /// when in time the stones will be close together
    fn estimate_rock(&self, maxt: usize) -> InitialCondition {
        let time_arr = {
            let n_time = 100.min(maxt);
            let d_time = maxt / n_time;
            let arr = (0..n_time).map(|n| (n * d_time) as i64);
            Array1::from_iter(arr)
        };
        let mut current_stone_pos = Array2::zeros((self.stones.len(), 3));
        let mut inv_dist_between_stones = Array2::zeros((self.stones.len(), self.stones.len()));
        let mut min_t = Array1::zeros(self.stones.len());
        let mut min_d = Array1::from_elem(self.stones.len(), f32::MAX);
        let mut e_pos = Array2::zeros((self.stones.len(), 3));

        // for each time compute distances and update estimates
        for t in time_arr.iter() {
            current_stone_pos
                .indexed_iter_mut()
                .for_each(|((istone, iaxis), p)| {
                    let s = &self.stones[istone];
                    *p = s.position.vec[iaxis] + s.velocity.vec[iaxis] * t;
                });
            for i in 0..self.stones.len() {
                for j in (i + 1)..self.stones.len() {
                    let mut d = 0.0;
                    for n in 0..3 {
                        d += ((current_stone_pos[[i, n]] - current_stone_pos[[j, n]]) as f32)
                            .powi(2);
                    }
                    d = 1.0 / d.sqrt();
                    inv_dist_between_stones[[i, j]] = d;
                    inv_dist_between_stones[[j, i]] = d;
                }
            }
            inv_dist_between_stones
                .axis_iter(Axis(0))
                .enumerate()
                .for_each(|(i, arr)| {
                    let d = arr.sum();
                    if i == 0 {
                        println!("time {} stone {} dist {}", t, i, d);
                    }
                    if d < min_d[i] {
                        if i == 0 {
                            println!("assign new best");
                        }
                        min_d[i] = d;
                        min_t[i] = *t;
                        // save best stone position
                        for n in 0..3 {
                            e_pos[[i, n]] = current_stone_pos[[i, n]];
                        }
                    }
                });
        }
        for i in 0..self.stones.len() {
            println!(
                "stone {} hits around time {} at pos {}",
                i,
                min_t[i],
                e_pos.slice(s![i, ..])
            );
        }

        todo!("Finish this");
        InitialCondition::new(Position::new([0, 0, 0]), Velocity::new([0, 0, 0]))
    }
    fn check_maxv(&self, maxv: i64) -> Option<InitialCondition> {
        let max_x = (-maxv..maxv)
            .cartesian_product(-maxv..maxv)
            .cartesian_product(0..maxv)
            .cartesian_product(0..self.stones.len())
            .map(|(((vy, vz), t0), i)| (maxv, vy, vz, t0, i));
        let min_x = (-maxv..maxv)
            .cartesian_product(-maxv..maxv)
            .cartesian_product(0..maxv)
            .cartesian_product(0..self.stones.len())
            .map(|(((vy, vz), t0), i)| (-maxv, vy, vz, t0, i));
        let max_y = (-maxv..maxv)
            .cartesian_product(-maxv..maxv)
            .cartesian_product(0..maxv)
            .cartesian_product(0..self.stones.len())
            .map(|(((vx, vz), t0), i)| (vx, maxv, vz, t0, i));
        let min_y = (-maxv..maxv)
            .cartesian_product(-maxv..maxv)
            .cartesian_product(0..maxv)
            .cartesian_product(0..self.stones.len())
            .map(|(((vx, vz), t0), i)| (vx, -maxv, vz, t0, i));
        let max_z = (-maxv..maxv)
            .cartesian_product(-maxv..maxv)
            .cartesian_product(0..maxv)
            .cartesian_product(0..self.stones.len())
            .map(|(((vx, vy), t0), i)| (vx, vy, maxv, t0, i));
        let min_z = (-maxv..maxv)
            .cartesian_product(-maxv..maxv)
            .cartesian_product(0..maxv)
            .cartesian_product(0..self.stones.len())
            .map(|(((vx, vy), t0), i)| (vx, vy, -maxv, t0, i));
        let max_t = (-maxv..maxv)
            .cartesian_product(-maxv..maxv)
            .cartesian_product(-maxv..maxv)
            .cartesian_product(0..self.stones.len())
            .map(|(((vx, vy), vz), i)| (vx, vy, vz, maxv, i));
        let rock = max_x
            .chain(min_x)
            .chain(max_y)
            .chain(min_y)
            .chain(max_z)
            .chain(min_z)
            .chain(max_t)
            .par_bridge()
            .map(|(vx, vy, vz, t0, i)| {
                let v = Velocity::new([vx, vy, vz]);
                let rock = solve_rock(t0 as usize, &self.stones[i], &v);
                if self.verify_rock(i, &rock) {
                    Some(rock)
                } else {
                    None
                }
            })
            .filter(|x| x.is_some())
            .collect::<Vec<_>>();
        if rock.is_empty() {
            None
        } else {
            rock[0]
        }
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
pub fn run(input: &str, maxt: usize) -> i64 {
    let hail = input.parse::<HailCloud>().unwrap();
    println!("{}", hail);
    let estimate = hail.estimate_rock(maxt);
    println!("estimate: {}", estimate);
    hail.run()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 10), 47);
    }
    #[test]
    fn test2() {
        let input = include_str!("data.txt");
        assert_eq!(super::run(input, 120_000_000_000), 0);
    }
}
