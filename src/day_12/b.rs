use std::{str::FromStr, time::Instant};

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
    record: &SpringRecord,
    solution: Vec<Condition>,
    child_operational: Option<SpringRecordIterator>,
    child_damaged: Option<SpringRecordIterator>,
}

// TODO working on making some kind of recursive iterator and thinking about how to
// not use a ton of memory. Maybe each recusive struct just has the proposed anser at a particular
// unknown but then you need to modify the record... still thinking on this
impl SpringRecordIterator {
    fn new(record: &SpringRecord) -> SpringRecordIterator {
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
        let record = (0..5)
            .map(|_| record.to_string())
            .collect::<Vec<_>>()
            .join(&"?");
        let groups = (0..5)
            .map(|_| groups.to_string())
            .collect::<Vec<_>>()
            .join(&",");
        println!("{} {}", record, groups);
        let record = record
            .chars()
            .map(|c| Condition::try_from(c).unwrap())
            .collect::<Vec<_>>();
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
    let mut count = 0;
    for line in input.lines() {
        println!("working on line {}", line);
        let now = Instant::now();
        count += line.parse::<SpringRecord>().unwrap().count_solutions();
        println!(
            "count {} took {} seconds",
            count,
            now.elapsed().as_secs_f32()
        );
    }
    count
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 525125);
    }
}
