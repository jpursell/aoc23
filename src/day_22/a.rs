use std::{fmt::Display, str::FromStr};

struct Brick {
    x: [u16; 2],
    y: [u16; 2],
    z: [u16; 2],
}
/// Parse something like 1,0,1 => x,y,z
fn parse_xyz(s: &str) -> Vec<u16> {
    s.split(",")
        .map(|x| x.parse::<u16>().unwrap())
        .collect::<Vec<_>>()
}

impl FromStr for Brick {
    type Err = &'static str;
    /// Parse something like 1,0,1~1,2,1 => x,y,z~x,y,z
    /// start_xyz~end_xyz (non-slice)
    /// Convert from non-slice to slice coords
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_end = s.split("~").map(|x| parse_xyz(x)).collect::<Vec<_>>();
        if start_end.len() != 2 {
            Err("Wrong number of parts")
        } else {
            if start_end.iter().any(|x| x.len() != 3) {
                Err("Wrong number of components")
            } else {
                let start = &start_end[0];
                let end = &start_end[1];
                Ok(Brick {
                    x: [start[0], end[0] + 1],
                    y: [start[1], end[1] + 1],
                    z: [start[2], end[2] + 1],
                })
            }
        }
    }
}

impl Display for Brick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{}~{},{},{}",
            self.x[0],
            self.y[0],
            self.z[0],
            self.x[1] - 1,
            self.y[1] - 1,
            self.z[1] - 1
        )
    }
}

pub fn run(input: &str) -> usize {
    let bricks = input
        .lines()
        .map(|line| line.parse::<Brick>().unwrap())
        .collect::<Vec<_>>();
    bricks.iter().for_each(|brick| {
        println!("{}", brick);
    });
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 5);
    }
}
