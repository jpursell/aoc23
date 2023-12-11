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
struct History<'a> {
    nodes: Vec<&'a String>,
}

impl<'a> History<'a> {
    fn new() -> Self {
        History { nodes: Vec::new() }
    }
    fn push(&mut self, node: &'a String) {
        self.nodes.push(node);
    }
    fn check(&self, len: usize) -> bool {
        if self.nodes.len() < len + 1 {
            return false;
        }
        self.nodes[(self.nodes.len() - 1) % len] == *self.nodes.last().unwrap()
    }
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    // fn find_loop(&self, start: &str, max_cycles: u64) -> Result<u64, &'static str> {
    //     let mut position = &start.to_string();
    //     for cycle in 0..max_cycles {
    //         for instruction in self.instructions.iter() {
    //             position = match instruction {
    //                 Direction::L => &self.nodes[position].0,
    //                 Direction::R => &self.nodes[position].1,
    //             }
    //         }
    //         if position == start {
    //             return Ok(cycle + 1);
    //         }
    //     }
    //     Err("Exceeded max cycles")
    // }

    // fn find_loops(&self) {
    //     for start in self.nodes.keys().filter(|s| s.ends_with("A")) {
    //         match self.find_loop(start, 10) {
    //             Ok(n_cycles) => {
    //                 println!("start {} has a loop with {} cycles", start, n_cycles);
    //             }
    //             Err(_) => {
    //                 println!("start {} no loop found", start);
    //             }
    //         }
    //     }
    // }

    fn count_steps(&self) -> u64 {
        let mut position = self
            .nodes
            .keys()
            .filter(|key| key.ends_with("A"))
            .collect::<Vec<_>>();
        let mut history = position
            .iter()
            .map(|node| {
                let mut h = History::new();
                h.push(node);
                h
            })
            .collect::<Vec<_>>();
        let mut steps = 0;
        for instruction in self.instructions.iter().cycle() {
            if position.iter().all(|key| key.ends_with("Z")) {
                break;
            }
            position
                .iter_mut()
                .zip(history.iter_mut())
                .for_each(|(pos, hist)| {
                    *pos = match instruction {
                        Direction::L => &self.nodes[*pos].0,
                        Direction::R => &self.nodes[*pos].1,
                    };
                    hist.push(pos);
                    if hist.check(self.instructions.len()) {
                        println!("found history loop {:?}", hist)
                    }
                });
            steps += 1;
        }
        steps
    }
    fn get_key_history(&self, key: &str) {
        let mut pos = &key.to_string();
        let mut history = History::new();
        history.push(pos);
        self.instructions.iter().cycle().for_each(|instruction|{
            pos = match instruction {
                Direction::L => self.nodes[pos].0,
                Direction::R => self.nodes[pos].1,
            };
            history.push(pos)
        })
        loop {

        }

    }
    fn count_steps_2(&self) -> u64 {
        let history = self.node.keys().filter(|key| key.ends_with("A")).map(self.get_key_history(key)).collect::<Vec<_>>();
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
