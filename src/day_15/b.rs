use std::{collections::HashMap, str::FromStr};

fn hash_str(s: &str) -> usize {
    let mut val = 0;
    s.chars().for_each(|c| {
        val += c as usize;
        val *= 17;
        val %= 256;
    });
    val
}

#[derive(Debug, Clone)]
struct Lens {
    label: String,
    lens_power: usize,
}
#[derive(Debug)]
enum Instruction {
    Add(Lens),
    Remove(String),
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        if s.len() < 3 {
            return Err("Too few characters");
        }
        let op_index = s
            .chars()
            .enumerate()
            .filter(|(_, c)| *c == '-' || *c == '=')
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        assert_eq!(op_index.len(), 1);
        let op_index = op_index[0];
        match s.chars().nth(op_index).unwrap() {
            '=' => {
                let (label, lens_power) = s.split_once("=").unwrap();
                Ok(Instruction::Add(Lens {
                    label: label.to_string(),
                    lens_power: lens_power.parse::<usize>().unwrap(),
                }))
            }
            '-' => {
                let label = s[0..op_index].to_string();
                Ok(Instruction::Remove(label))
            }
            _ => panic!(),
        }
    }
}
#[derive(Debug, Default)]
struct Box {
    slots: Vec<Lens>,
}

impl Box {
    fn remove(&mut self, label: &str) {
        let to_remove = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, x)| x.label == label)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        assert!(to_remove.len() <= 1);
        to_remove.iter().for_each(|i| {
            self.slots.remove(*i);
        });
    }
    fn add(&mut self, lens: Lens) {
        let matching = self
            .slots
            .iter()
            .enumerate()
            .filter(|(_, x)| x.label == lens.label)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        match matching.len() {
            0 => {
                self.slots.push(lens);
            }
            1 => {
                self.slots[matching[0]] = lens;
            }
            _ => panic!(),
        }
    }
    fn focusing_power(&self) -> usize {
        self.slots
            .iter()
            .enumerate()
            .map(|(i, x)| (i + 1) * x.lens_power)
            .sum()
    }
}

#[derive(Debug, Default)]
struct Boxes {
    boxes: HashMap<usize, Box>,
}

impl Boxes {
    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .map(|(i, b)| (i + 1) * b.focusing_power())
            .sum()
    }
    fn execute(&mut self, instruction: Instruction) {
        let hash = match &instruction {
            Instruction::Add(lens) => hash_str(&lens.label),
            Instruction::Remove(label) => hash_str(&label),
        };
        if !self.boxes.contains_key(&hash) {
            self.boxes.insert(hash, Box::default());
        }
        let b = self.boxes.get_mut(&hash).unwrap();
        match instruction {
            Instruction::Add(lens) => {
                b.add(lens);
            }
            Instruction::Remove(label) => {
                b.remove(&label);
            }
        }
    }
}

pub fn run(input: &str) -> usize {
    let mut boxes = Boxes::default();
    input
        .split(",")
        .map(|s| s.parse::<Instruction>().unwrap())
        .for_each(|x| {
            boxes.execute(x);
        });
    boxes.focusing_power()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 145);
    }
    #[test]
    fn test_hash_1() {
        assert_eq!(super::hash_str("rn"), 0);
    }
    #[test]
    fn test_hash_2() {
        assert_eq!(super::hash_str("qp"), 1);
    }
}
