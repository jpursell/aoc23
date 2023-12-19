use std::{collections::LinkedList, str::FromStr};

fn hash_str(s: &str) -> usize {
    let mut val = 0;
    s.chars().for_each(|c| {
        val += c as usize;
        val *= 17;
        val %= 256;
    });
    val
}

struct Lens {
    label: String,
    lens_power: usize,
}
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
        match s.chars().nth(2).unwrap() {
            '=' => {
                let (label, lens_power) = s.split_once("=").unwrap();
                Ok(Instruction::Add(Lens {
                    label: label.to_string(),
                    lens_power: lens_power.parse::<usize>().unwrap(),
                }))
            }
            '-' => {
                let label = s[0..2].to_string();
                Ok(Instruction::Remove(label))
            }
            _ => panic!(),
        }
    }
}
struct Box {
    slots: Vec<Lens>,
}

impl Box {
    fn remove(&mut self, label: &str) {
        self.slots = self
            .slots
            .iter()
            .filter(|x| x.label != label)
            .map(|x| *x)
            .collect::<Vec<_>>();
    }
    fn add(&mut self, lens: Lens) {
        let matching = self
            .slots
            .iter()
            .enumerate()
            .filter(|(i, x)| x.label == lens.label)
            .map(|(i, x)| i)
            .collect::<Vec<_>>();
        match matching.len() {
            0 => todo!(),
            1 => todo!(),
            _ => panic!(),
        }
    }
}

struct Boxes {
    boxes: HashMap<usize, Box>,
}

pub fn run(input: &str) -> usize {
    input.split(",").map(|s| hash_str(s)).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 1320);
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
