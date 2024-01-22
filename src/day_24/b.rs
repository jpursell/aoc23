use std::{fmt::Display, str::FromStr, time::Instant};

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
    p: Vec<Vec<f64>>,
    v: Vec<Vec<f64>>,
}
impl HailCloud {
    fn new(stones: Vec<InitialCondition>) -> HailCloud {
        let p = stones
            .iter()
            .map(|s| s.position.vec.iter().map(|x| *x as f64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let v = stones
            .iter()
            .map(|s| s.velocity.vec.iter().map(|x| *x as f64).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        HailCloud { stones, p, v }
    }

    /// Solve for rock that will pass through all hail positions
    /// and return sum of initial position coords
    fn run(&self) -> i64 {
        self.estimate_rock().position_sum()
    }
    fn estimate_error(&self, rp: &[f64; 3], rv: &[f64; 3], t: &[f64]) -> f64 {
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
    fn estimate_rock(&self) -> InitialCondition {
        let mut rp = [0_f64;3];
        let mut rv = [0_f64;3];
        let mut t = vec![0_f64;self.stones.len()];

        // use gradient descent to refine solution
        let mut rock;
        let mut last_rock = InitialCondition::default();
        let mut it = 0;
        let start = Instant::now();
        let mut debug: bool;
        loop {
            it += 1;
            if it == 500 {
                dbg!(rp);
                dbg!(rv);
                dbg!(t);
                panic!();
            }
            debug = false;
            if it % 10 == 0 {
                debug = true;
                let error = self.estimate_error(&rp, &rv, &t);
                println!(
                    "it {}, t: {} error: {:.2e} last_rock: {}",
                    it,
                    start.elapsed().as_secs(),
                    error,
                    last_rock
                );
            }
            // newton's method of finding local minima
            // update time
            let mut max_time_change: f64 = 0.0;
            for i in 0..self.stones.len() {
                let p = &self.p[i];
                let v = &self.v[i];
                let mut ddt: f64 = 0.0;
                let mut dt: f64 = 0.0;
                for n in 0..3 {
                    ddt += 2.0 * ((rv[n] - v[n]).powi(2));
                    dt += (-2.0 * rv[n] + 2.0 * v[n]) * (p[n] - rp[n] - rv[n] * t[i] + t[i] * v[n]);
                }
                if debug {
                    max_time_change = max_time_change.max((dt / ddt).abs())
                }
                t[i] -= dt / ddt;
            }
            if debug {
                print!("dt: {} ", max_time_change);
            }
            // update rp
            {
                let mut max_pos_change: f64 = 0.0;
                let ddp: f64 = 2.0 * self.stones.len() as f64;
                let mut dp = [0.0_f64; 3];
                for i in 0..self.stones.len() {
                    let p = &self.p[i];
                    let v = &self.v[i];
                    for n in 0..3 {
                        dp[n] += -2.0 * p[n] + 2.0 * rp[n] + 2.0 * rv[n] * t[i] - 2.0 * t[i] * v[n];
                    }
                }
                for n in 0..3 {
                    if debug {
                        max_pos_change = max_pos_change.max((dp[n] / ddp).abs());
                    }
                    rp[n] -= dp[n] / ddp;
                }
                if debug {
                    print!("dp: {} ", max_pos_change);
                }
            }
            // update rv
            {
                let mut ddv = [0.0_f64; 3];
                let mut dv = [0.0_f64; 3];
                let mut max_v_change: f64 = 0.0;
                for i in 0..self.stones.len() {
                    let p = &self.p[i];
                    let v = &self.v[i];
                    for n in 0..3 {
                        ddv[n] += 2.0 * t[i].powi(2);
                        dv[n] += -2.0 * t[i] * (p[n] - rp[n] - rv[n] * t[i] + t[i] * v[n]);
                    }
                }
                for n in 0..3 {
                    if debug {
                        max_v_change = max_v_change.max((dv[n] / ddv[n]).abs());
                    }
                    rv[n] -= dv[n] / ddv[n];
                }
                if debug {
                    println!("dv: {} ", max_v_change);
                }
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
            // if rock != last_rock {
            if self.verify_rock(&rock, it == 499) {
                println!("Solved after it: {} rock: {}", it, rock);
                break;
            }
            // }
            last_rock = rock;
        }

        rock
    }
    /// Return true if rock intersects all stones. Assume stones at i and j are already checked
    fn verify_rock(&self, rock: &InitialCondition, debug: bool) -> bool {
        for k in 0..self.stones.len() {
            // stone_loc = stone_pos + stone_vel * t
            // rock_loc = rock_pos + rock_vel * t
            // stone_pos + stone_vel * t = rock_pos + rock_vel * t
            // stone_pos - rock_pos = t * (rock_vel - stone_vel)
            // t = (stone_pos - rock_pos) / (rock_vel - stone_vel)
            let stone = &self.stones[k];
            let bad_x = rock.velocity.vec[0] as i64 == stone.velocity.vec[0] as i64;
            let t = if bad_x {
                (stone.position.vec[1] as i64 - rock.position.vec[1] as i64)
                    / (rock.velocity.vec[1] as i64 - stone.velocity.vec[1] as i64)
            } else {
                (stone.position.vec[0] as i64 - rock.position.vec[0] as i64)
                    / (rock.velocity.vec[0] as i64 - stone.velocity.vec[0] as i64)
            };
            if t < 0 {
                if debug {
                    println!("negative time stone: {}", k);
                }
                return false;
            }
            for n in 0..3 {
                if rock.position.vec[n] as i64 + rock.velocity.vec[n] as i64 * t
                    != stone.position.vec[n] as i64 + stone.velocity.vec[n] as i64 * t
                {
                    if debug {
                        println!("non-match at stone {} dim {}", k, n);
                    }
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
pub fn run(input: &str) -> i64 {
    let hail = input.parse::<HailCloud>().unwrap();
    hail.run()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 47);
    }
    #[test]
    fn test2() {
        let input = include_str!("data.txt");
        assert_eq!(super::run(input), 0);
    }
}
