use std::str::FromStr;

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
    pos: u64,
    tot: u64,
}

impl SpringRecordIterator {
    fn new(record: &SpringRecord) -> SpringRecordIterator {
        let tot = 2_u64.pow(
            record
                .record
                .iter()
                .filter(|c| **c == Condition::Unknown)
                .count() as u32,
        );
        SpringRecordIterator {
            record: record.record.clone(),
            pos: 0,
            tot,
        }
    }
}

impl Iterator for SpringRecordIterator {
    type Item = Vec<Condition>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        todo!()
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
        let groups = groups
            .split(",")
            .map(|num| num.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        Ok(SpringRecord { record, groups })
    }
}

impl SpringRecord {
    fn count_solutions(&self) -> u64 {
        0
    }

    fn iter(&self) -> SpringRecordIterator {
        SpringRecordIterator::new(self)
    }
}

pub fn run(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.parse::<SpringRecord>().unwrap())
        .inspect(|sr| {
            dbg!(&sr);
        })
        .map(|sr| sr.count_solutions())
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 21);
    }
}
