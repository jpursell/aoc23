use std::str::FromStr;

#[derive(Debug)]
enum Direction {
    U,
    D,
    L,
    R,
}

impl FromStr for Direction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Direction::U),
            "D" => Ok(Direction::D),
            "L" => Ok(Direction::L),
            "R" => Ok(Direction::R),
            _ => Err("Unknown direction"),
        }
    }
}

#[derive(Debug)]
struct DigInstruction {
    direction: Direction,
    distance: usize,
    color: String,
}
impl FromStr for DigInstruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // R 10 (#ffffff)
        let mut s = s.split_whitespace();
        let direction = s.next().unwrap().parse::<Direction>().unwrap();
        let distance = s.next().unwrap().parse::<usize>().unwrap();
        let color = s.next().unwrap().to_string();
        Ok(DigInstruction {
            direction,
            distance,
            color,
        })
    }
}
#[derive(Debug)]
struct DigPlan {
    instructions: Vec<DigInstruction>,
}

impl FromStr for DigPlan {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DigPlan {
            instructions: s
                .lines()
                .map(|line| line.parse::<DigInstruction>().unwrap())
                .collect::<Vec<_>>(),
        })
    }
}

pub fn run(input: &str) -> usize {
    let plan = input.parse::<DigPlan>().unwrap();
    dbg!(&plan);
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 0);
    }
}
