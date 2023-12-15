use std::{str::FromStr, time::Instant};

use itertools::{Combinations, Itertools};
use rayon::iter::ParallelIterator;

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
    check_cache: Option<bool>,
    complete_cache: Option<bool>,
    done: bool,
}

impl SpringRecordIterator {
    /// Create new recursive spring record iterator
    ///
    /// This iterates through solutions for the problem.
    /// The solution at the top level should be an empty Vec
    fn new(record: &SpringRecord, solution: Vec<Condition>) -> SpringRecordIterator {
        SpringRecordIterator {
            record,
            solution,
            child_operational: None,
            child_damaged: None,
            check_cache: None,
            complete_cache: None,
            done: false,
        }
    }

    /// Check if there are no more unknowns
    fn complete(&mut self) -> bool {
        if let Some(completed) = self.complete_cache {
            return completed;
        }

        // todo complete function
        todo!();
        let ret = false;

        self.complete_cache = Some(ret);
        ret
    }

    /// Check to see if current solution is possible. Can handle unknowns.
    ///
    /// The answer should be saved in a bool
    fn check(&self) -> bool {
        if let Some(ret) = self.check_cache {
            return ret;
        }

        todo!();
        // TODO implement check that can handle unknowns
        let ret = false;

        // let groups = condition
        //     .split(|&c| c == Condition::Operational)
        //     .map(|c| c.len())
        //     .filter(|&n| n > 0)
        //     .collect::<Vec<_>>();
        // if groups.len() != self.groups.len() {
        //     return false;
        // }
        // groups
        //     .iter()
        //     .zip(self.groups.iter())
        //     .all(|(&x, &y)| x == y as usize)
        self.check_cache = Some(ret);
        ret
    }
}

impl Iterator for SpringRecordIterator {
    type Item = Vec<Condition>;
    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if !self.check() || self.done {
            return None;
        }

        if self.complete() {
            self.done = true;
            return Some(self.solution);
        } else {
            // TODO Add code for children
            todo!()
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
