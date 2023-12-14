use std::str::FromStr;

use itertools::{Combinations, Itertools};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Condition {
    Operational,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Condition {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, <Self as TryFrom<char>>::Error> {
        match c {
            '.' => Ok(Condition::Operational),
            '#' => Ok(Condition::Damaged),
            '?' => Ok(Condition::Unknown),
            _ => Err("Unknown char"),
        }
    }
}

#[derive(Debug)]
struct SpringRecord {
    record: Vec<Condition>,
    groups: Vec<u32>,
}

struct SpringRecordIterator {
    record: Vec<Condition>,
    groups: Vec<u32>,
    combinations: Combinations<std::vec::IntoIter<usize>>,
}

impl SpringRecordIterator {
    fn new(record: &SpringRecord) -> SpringRecordIterator {
        let loc = record
            .record
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == Condition::Unknown)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let groups = record.groups.clone();
        let record = record
            .record
            .iter()
            .map(|&c| {
                if c == Condition::Unknown {
                    Condition::Operational
                } else {
                    c
                }
            })
            .collect::<Vec<_>>();
        let mut tot = groups.iter().map(|&x| x as usize).sum();
        tot -= record.iter().filter(|&&c| c == Condition::Damaged).count();
        let combinations = loc.into_iter().combinations(tot);

        SpringRecordIterator {
            record,
            groups,
            combinations,
        }
    }

    fn check(&self, condition: &Vec<Condition>) -> bool {
        let groups = condition
            .split(|&c| c == Condition::Operational)
            .map(|c| c.len())
            .filter(|&n| n > 0)
            .collect::<Vec<_>>();
        if groups.len() != self.groups.len() {
            return false;
        }
        groups
            .iter()
            .zip(self.groups.iter())
            .all(|(&x, &y)| x == y as usize)
    }
}

impl Iterator for SpringRecordIterator {
    type Item = Vec<Condition>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        loop {
            let Some(loc) = self.combinations.next() else {
                return None;
            };
            let mut out = self.record.clone();
            loc.iter().for_each(|&i| out[i] = Condition::Damaged);
            if self.check(&out) {
                return Some(out);
            }
        }
    }
}

impl FromStr for SpringRecord {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
        let (record, groups) = line.split_once(" ").unwrap();
        let record = record
            .chars()
            .map(|c| Condition::try_from(c).unwrap())
            .collect::<Vec<_>>();
        // todo x5 these
        let groups = groups
            .split(",")
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Ok(SpringRecord { record, groups })
    }
}

impl SpringRecord {
    fn count_solutions(&self) -> usize {
        self.iter().count()
    }

    fn iter(&self) -> SpringRecordIterator {
        SpringRecordIterator::new(self)
    }
}

pub fn run(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<SpringRecord>().unwrap())
        .map(|sr| sr.count_solutions())
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 525125);
    }
}
