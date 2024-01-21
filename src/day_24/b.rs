use std::{fmt::Display, str::FromStr, time::Instant};

use ndarray::{s, Array1, Array2, Axis};
use polyfit_rs::polyfit_rs::polyfit;
use rand::random;

#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Debug, Clone, Copy)]
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Debug, Clone, Copy)]
struct Velocity {
    vec: [i64; 3],
}
impl Velocity {
    fn new(vec: [i64; 3]) -> Velocity {
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
#[derive(PartialEq, Eq, PartialOrd, Ord, Default, Clone, Copy, Debug)]
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
struct HailCloud {
    stones: Vec<InitialCondition>,
}
impl HailCloud {
    fn new(stones: Vec<InitialCondition>) -> HailCloud {
        HailCloud { stones }
    }

    /// Solve for rock that will pass through all hail positions
    /// and return sum of initial position coords
    fn run(&self, maxt: usize, lr: f64, dropout: f64) -> i64 {
        self.estimate_rock(maxt, lr, dropout).position_sum()
    }
    fn estimate_error(&self, rp: &[f64;3], rv: &[f64;3], t:&[f64]) -> f64 {
        let mut error = 0.0;
        for i in 0..self.stones.len() {
            let p = &self.stones[i].position.vec;
            let v = &self.stones[i].velocity.vec;
            for n in 0..3 {
                let rock_p = rp[n] + rv[n] * t[i];
                let stone_p = p[n] as f64 + v[n] as f64 * t[i];
                error += (stone_p - rock_p).powi(2);
            }
        }
        error
    }
    /// Make a guess at the rock's initial conditions by observing
    /// when in time the stones will be close together
    fn estimate_rock(&self, maxt: usize, lr: f64, dropout: f64) -> InitialCondition {
        let time_arr = {
            let n_time = 1000.min(maxt);
            let d_time = maxt / n_time;
            let arr = (0..n_time).map(|n| (n * d_time) as i64);
            Array1::from_iter(arr)
        };
        let mut current_stone_pos = Array2::zeros((self.stones.len(), 3));
        let mut inv_dist_between_stones = Array2::zeros((self.stones.len(), self.stones.len()));
        let mut min_t = Array1::zeros(self.stones.len());
        let mut min_d = Array1::from_elem(self.stones.len(), f64::MIN);
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
                        d += ((current_stone_pos[[i, n]] - current_stone_pos[[j, n]]) as f64)
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
                    if d > min_d[i] {
                        min_d[i] = d;
                        min_t[i] = *t as f64;
                        // save best stone position
                        for n in 0..3 {
                            e_pos[[i, n]] = current_stone_pos[[i, n]] as f64;
                        }
                    }
                });
        }

        let x_result = polyfit(&(min_t.to_vec()), &(e_pos.slice(s![.., 0]).to_vec()), 1).unwrap();
        let y_result = polyfit(&(min_t.to_vec()), &(e_pos.slice(s![.., 1]).to_vec()), 1).unwrap();
        let z_result = polyfit(&(min_t.to_vec()), &(e_pos.slice(s![.., 2]).to_vec()), 1).unwrap();
        let mut rp = [x_result[0], y_result[0], z_result[0]];
        let mut rv = [x_result[1], y_result[1], z_result[1]];
        let mut t = min_t.to_vec();

        // use gradient descent to refine solution
        let mut grad_rp = [0.0_f64; 3];
        let mut grad_rv = [0.0_f64; 3];
        let mut grad_t = vec![0.0_f64; self.stones.len()];
        let p64 = self
            .stones
            .iter()
            .map(|s| s.position.vec.iter().map(|&x| x as f64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let v64 = self
            .stones
            .iter()
            .map(|s| s.velocity.vec.iter().map(|&x| x as f64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut rock;
        let mut last_rock = InitialCondition::default();
        let mut it = 0;
        let start = Instant::now();
        let mut last = Instant::now();
        loop {
            it += 1;
            if last.elapsed().as_secs_f32() > 5.0 {
                last = Instant::now();
                let error = self.estimate_error(&rp, &rv, &t);
                println!("it {}, t: {} error: {:.2e}", it, start.elapsed().as_secs(), error);
            }
            for i in 0..3 {
                grad_rp[i] = 0.0;
                grad_rv[i] = 0.0;
            }
            for i in 0..self.stones.len() {
                grad_t[i] = 0.0;
            }
            for i in 0..self.stones.len() {
                if random::<f64>() < dropout {
                    continue;
                }
                let p = &p64[i];
                let v = &v64[i];
                for n in 0..3 {
                    // de^2/drp = -2*p + 2*rp + 2*rv*t - 2*t*v
                    grad_rp[n] +=
                        -2.0 * p[n] + 2.0 * rp[n] + 2.0 * rv[n] * t[i] - 2.0 * t[i] * v[n];
                    // de^2/drv = -2*t*(p - rp - rv*t + t*v)
                    grad_rv[n] += -2.0 * t[i] * (p[n] - rp[n] - rv[n] * t[i] + t[i] * v[n]);
                    // de^2/dt = (-2*rv + 2*v)*(p - rp - rv*t + t*v)
                    grad_t[i] +=
                        (-2.0 * rv[n] + 2.0 * v[n]) * (p[n] - rp[n] - rv[n] * t[i] + t[i] * v[n]);
                }
            }
            let find_max = |x: &[f64]| x.iter().fold(0.0_f64, |a: f64, &b| a.max(b.abs()));
            // let max_grad = find_max(&grad_t)
            //     .max(find_max(&grad_rp))
            //     .max(find_max(&grad_rv));
            let max_grad = find_max(&grad_rv);
            let lr = (lr).min(0.01_f64 / max_grad);
            for i in 0..self.stones.len() {
                t[i] -= grad_t[i] * lr;
            }
            for i in 0..3 {
                rp[i] -= grad_rp[i] * lr;
                rv[i] -= grad_rv[i] * lr;
            }
            if !rp.iter().all(|x| x.is_finite()) {
                dbg!(&rp);
                panic!("rp not finite");
            }
            let rock_position = Position::new([
                rp[0].round() as i64,
                rp[1].round() as i64,
                rp[2].round() as i64,
            ]);
            if !rv.iter().all(|x| x.is_finite()) {
                panic!("rv not finite");
            }
            let rock_velocity = Velocity::new([
                rv[0].round() as i64,
                rv[1].round() as i64,
                rv[2].round() as i64,
            ]);
            rock = InitialCondition::new(rock_position, rock_velocity);
            if rock != last_rock {
                if self.verify_rock(&rock) {
                    println!("Solved rock {}", rock);
                    break;
                }
            }
            last_rock = rock;
        }

        rock
    }
    /// Return true if rock intersects all stones. Assume stones at i and j are already checked
    fn verify_rock(&self, rock: &InitialCondition) -> bool {
        for k in 0..self.stones.len() {
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
pub fn run(input: &str, maxt: usize, lr: f64, dropout: f64) -> i64 {
    let hail = input.parse::<HailCloud>().unwrap();
    hail.run(maxt, lr, dropout)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 10, 0.01, 0.5), 47);
    }
    #[test]
    fn test2() {
        let input = include_str!("data.txt");
        assert_eq!(super::run(input, 1_200_000_000_000, 0.01, 0.50), 0);
    }
}
