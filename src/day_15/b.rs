use std::collections::LinkedList;

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
struct Box {
    slots: Vec<Lens>,
}

impl Box {
    fn remove(&mut self, label: &str) {
        self.slots = self
            .slots
            .iter()
            .filter(|x| x.label != label)
            .collect::<Vec<_>>();
    }
    fn add(lens: Lens) {}
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
