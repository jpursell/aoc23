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
    instruction_len: usize,
    complete: bool,
}

impl<'a> History<'a> {
    fn new(instruction_len: usize) -> Self {
        History {
            nodes: Vec::new(),
            instruction_len,
            complete: false,
        }
    }
    fn from_map(map: &Map) -> Self {
        Self::new(map.instructions.len())
    }
    fn push(&mut self, node: &'a String) {
        self.nodes.push(node);
        self.update();
    }
    fn check(&self) -> bool {
        self.complete
    }
    fn cycle_len(&self) -> usize {
        self.instruction_len
    }
    fn cycle_start(&self) -> usize {
        (self.nodes.len() - 1) % self.cycle_len()
    }
    fn update(&self) -> bool {
        if self.nodes.len() < self.instruction_len + 1 {
            return false;
        }
        let pos = self.cycle_start();
        let old_node = self.nodes[pos];
        let latest_node = *self.nodes.last().unwrap();
        let ret = old_node == latest_node;
        if ret {
            println!(
                "history match on {} and {} comparing last to pos {}",
                old_node, latest_node, pos
            );
        }
        ret
    }
}

struct Historian<'a> {
    history: History<'a>,
    pos: usize,
    cycle: usize,
    cycle_start: usize,
    cycle_len: usize,
    end_pos: Vec<usize>,
}

impl<'a> Historian<'a> {
    fn new(history: History<'a>) -> Self {
        assert!(history.check());
        let cycle_len = history.cycle_len();
        let cycle_start = history.cycle_start();
        let end_pos = history
            .nodes
            .iter()
            .enumerate()
            .filter(|(num, key)| key.ends_with("Z"))
            .map(|(num, key)| num)
            .collect::<Vec<usize>>();
        Historian {
            history,
            pos: 0,
            cycle: 0,
            cycle_start,
            cycle_len,
            end_pos,
        }
    }
}

impl<'a> TryFrom<History<'a>> for Historian<'a> {
    type Error = &'static str;
    fn try_from(hist: History<'a>) -> Result<Self, <Self as TryFrom<History<'a>>>::Error> {
        if !hist.check() {
            Err("Incomplete history")
        } else {
            Ok(Historian::new(hist))
        }
    }
}

impl Iterator for Historian<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let ret = Some(self.end_pos[self.pos] + self.cycle_len * self.cycle);
        self.pos += 1;
        if self.pos == self.end_pos.len() {
            self.pos = 0;
            self.cycle += 1;
        }
        ret
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
                let mut h = History::from_map(self);
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
                    if hist.check() {
                        println!("found history loop {:?}", hist)
                    }
                });
            steps += 1;
        }
        steps
    }

    fn get_key_history<'a>(&'a self, key: &'a String) -> History<'_> {
        let mut pos = key;
        let mut history = History::from_map(self);
        history.push(pos);
        for instruction in self.instructions.iter().cycle() {
            pos = match instruction {
                Direction::L => &self.nodes[pos].0,
                Direction::R => &self.nodes[pos].1,
            };
            history.push(pos);
            if history.check() {
                break;
            }
        }
        history
    }

    fn count_steps_2(&self) -> u64 {
        let history = self
            .nodes
            .keys()
            .filter(|key| key.ends_with("A"))
            .map(|key| self.get_key_history(key))
            .collect::<Vec<_>>();
        dbg!(history);
        // TODO need to make historians and use them to keep calling next on the smallest ones until they all match
        todo!();
        0
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
    input.parse::<Map>().unwrap().count_steps_2()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_b_data.txt");
        assert_eq!(super::run(input), 6);
    }
}
