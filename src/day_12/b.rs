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
    /// Check if there are no more unknowns
    fn complete(&self, solution: &Vec<Condition>) -> bool {
        // todo complete function
        todo!();
    }

    /// Check to see if current solution is possible. Can handle unknowns.
    ///
    /// The answer should be saved in a bool
    fn check(&self, solution: &Vec<Condition>) -> bool {
        todo!();
        // TODO implement check that can handle unknowns

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
    }

    fn append_solution(&self, solution: &mut Vec<Condition>, condition: &Condition) {
        todo!()
    }
    fn undo_solution(&self, solution: &mut Vec<Condition>) {
        todo!()
    }
    fn count_solutions(&self, solution: &mut Vec<Condition>) -> usize {
        if !self.check(solution) {
            return 0;
        }

        if self.complete() {
            return 1;
        } else {
            self.append_solution(solution, &Condition::Damaged);
            let mut count = self.count_solutions(solution);
            self.undo_solution(solution);

            self.append_solution(solution, &Condition::Operational);
            count += self.count_solutions(solution);
            self.undo_solution(solution);

            return count;
        }
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
