use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Direction {
    L,
    R,
}

impl TryFrom<char> for Direction {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, <Self as TryFrom<char>>::Error> {
        match c {
            'L' => Ok(Direction::L),
            'R' => Ok(Direction::R),
            _ => Err("Got bad direction char"),
        }
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    fn count_steps(&self) -> u64 {
        let mut position = self
            .nodes
            .keys()
            .filter(|key| key.ends_with("A"))
            .collect::<Vec<_>>();
        let mut steps = 0;
        for instruction in self.instructions.iter().cycle() {
            if position.iter().all(|key| key.ends_with("Z")) {
                break;
            }
            position.iter_mut().for_each(|pos| {
                *pos = match instruction {
                    Direction::L => &self.nodes[*pos].0,
                    Direction::R => &self.nodes[*pos].1,
                };
            });
            steps += 1;
        }
        steps
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(val: &str) -> Result<Map, <Self as FromStr>::Err> {
        let instructions = val
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| Direction::try_from(c).unwrap())
            .collect::<Vec<_>>();

        let nodes = val
            .lines()
            .skip(2)
            .map(|line| {
                let (input, output) = line.split_once(" = ").unwrap();
                let output = &output[1..output.len() - 1];
                let (left_output, right_output) = output.split_once(", ").unwrap();
                (
                    input.to_string(),
                    (left_output.to_string(), right_output.to_string()),
                )
            })
            .collect::<HashMap<_, _>>();

        Ok(Map {
            instructions,
            nodes,
        })
    }
}

pub fn run(input: &str) -> u64 {
    input.parse::<Map>().unwrap().count_steps()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_b_data.txt");
        assert_eq!(super::run(input), 6);
    }
}
