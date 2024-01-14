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
    // fn norm_xy(&self) -> [f64; 2] {
    //     let out = [self.vec[0] as f64, self.vec[1] as f64];
    //     let norm = f64::sqrt(out[0].powi(2) + out[1].powi(2));
    //     [out[0] / norm, out[1] / norm]
    // }
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
struct Line {
    m: f64,
    b: f64,
}
impl Line {
    fn new(m: f64, b: f64) -> Line {
        Line { m, b }
    }
}
impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y = {} x + {}", self.m, self.b)
    }
}
impl TryFrom<InitialCondition> for Line {
    type Error = ();
    fn try_from(value: InitialCondition) -> Result<Self, Self::Error> {
        // pos x,y moving at vel dx,dy
        // y = mx + b
        // when x is x we get
        // y = b
        let x = value.position.vec[0] as f64;
        let y = value.position.vec[1] as f64;
        let dx = value.velocity.vec[0] as f64;
        let dy = value.velocity.vec[1] as f64;
        let b = {
            // we want to find the value for t when x is 0
            // newx = x + dx * t
            // -x = dx * t
            // t = -x / dx
            let t = -x / dx;
            y + dy * t
        };
        if dx == 0_f64 {
            return Err(());
        }
        let m = dy / dx;
        Ok(Line::new(m, b))
    }
}
fn find_intersection(
    a_cond: &InitialCondition,
    a_line: &Line,
    b_cond: &InitialCondition,
    b_line: &Line,
) -> Option<[f64; 2]> {
    if a_line.m == b_line.m {
        return None;
    }
    // y = mx + b
    // m0 * x + b0 - m1 * x - b1 = 0
    // m0 * x - m1 * x = b1 - b0
    // x (m0 - m1) = b1 - b0
    // x = (b1 - b0) / (m0 - m1)
    let x = (b_line.b - a_line.b) / (a_line.m - b_line.m);
    // x = pos_x + vel_x * t
    // t = (x - pos_x) / vel_x
    let t_a = (x - a_cond.position.vec[0] as f64) / a_cond.velocity.vec[0] as f64;
    let t_b = (x - b_cond.position.vec[0] as f64) / b_cond.velocity.vec[0] as f64;
    if t_a < 0.0 || t_b < 0.0 {
        return None;
    }
    let y_a = a_line.m * x + a_line.b;
    // let y_b = b.m * x + b.b;
    // assert_eq!(y_a, y_b);
    Some([x, y_a])
}
struct HailCloud {
    stones: Vec<InitialCondition>,
    stone_lines: Vec<Line>,
}
impl HailCloud {
    fn new(stones: Vec<InitialCondition>) -> HailCloud {
        let stone_lines = stones
            .iter()
            .map(|x| Line::try_from(*x).unwrap())
            .collect::<Vec<_>>();
        HailCloud {
            stones,
            stone_lines,
        }
    }
    fn count_intersections_in_test_area(&self, min_test: f64, max_test: f64) -> usize {
        let mut count = 0;
        for i in 0..self.stones.len() {
            for j in i + 1..self.stones.len() {
                let intersection = find_intersection(
                    &self.stones[i],
                    &self.stone_lines[i],
                    &self.stones[j],
                    &self.stone_lines[j],
                );
                if intersection.is_none() {
                    continue;
                }
                println!(
                    "\nintersect\n\t{}\n\t{}\n\t{:?}",
                    self.stones[i], self.stones[j], intersection
                );
                let intersection = intersection.unwrap();
                if intersection[0] >= min_test
                    && intersection[0] <= max_test
                    && intersection[1] >= min_test
                    && intersection[1] <= max_test
                {
                    println!("inside");
                    count += 1;
                }
            }
        }
        count
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
        for (stone, line) in self.stones.iter().zip(self.stone_lines.iter()) {
            writeln!(f, "{} -> {}", stone, line)?
        }
        Ok(())
    }
}
pub fn run(input: &str, min_test: f64, max_test: f64) -> usize {
    let hail = input.parse::<HailCloud>().unwrap();
    println!("{}", hail);
    hail.count_intersections_in_test_area(min_test, max_test)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, 7_f64, 27_f64), 2);
    }
}
