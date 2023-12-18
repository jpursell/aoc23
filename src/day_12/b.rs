use rayon::prelude::*;
use simple_tqdm::ParTqdm;
use std::{str::FromStr, time::Instant};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Condition {
    Opr,
    Dam,
    Unk,
}

impl TryFrom<char> for Condition {
    type Error = &'static str;
    fn try_from(c: char) -> Result<Self, <Self as TryFrom<char>>::Error> {
        match c {
            '.' => Ok(Condition::Opr),
            '#' => Ok(Condition::Dam),
            '?' => Ok(Condition::Unk),
            _ => Err("Unknown char"),
        }
    }
}

#[derive(Clone, Debug)]
struct Solution {
    extra_space: usize,
    record_pos: usize,
    group_pos: usize,
}

impl Solution {
    fn new(sr: &SpringRecord) -> Solution {
        let extra_space = sr.record.len() - sr.groups.iter().sum::<usize>() - (sr.groups.len() - 1);
        Solution {
            extra_space,
            record_pos: 0,
            group_pos: 0,
        }
    }

    fn complete(&self, sr: &SpringRecord) -> bool {
        self.group_pos == sr.groups.len()
            && self.record_pos == sr.record.len()
            && self.extra_space == 0
    }

    /// Try to create new solution with `n_spaces` extra spaces in the next spot
    fn push(&self, n_spaces: usize, sr: &SpringRecord) -> Option<Solution> {
        let mut s = self.clone();
        if s.extra_space < n_spaces {
            panic!()
        }
        s.extra_space -= n_spaces;
        // check extra spaces
        for _ in 0..n_spaces {
            if s.record_pos == sr.record.len() {
                return None;
            }
            if sr.record[s.record_pos] == Condition::Dam {
                return None;
            }
            s.record_pos += 1;
        }
        // check to see if done
        if s.group_pos == sr.groups.len() {
            if s.complete(sr) {
                return Some(s);
            } else {
                return None;
            }
        }
        // check group add
        for _ in 0..sr.groups[s.group_pos] {
            if s.record_pos == sr.record.len() {
                return None;
            }
            if sr.record[s.record_pos] == Condition::Opr {
                return None;
            }
            s.record_pos += 1;
        }
        s.group_pos += 1;
        // check to see if done
        if s.complete(sr) {
            return Some(s);
        }
        // put in final .
        if s.record_pos == sr.record.len() {
            return None;
        }
        if sr.record[s.record_pos] == Condition::Dam {
            return None;
        }
        s.record_pos += 1;
        // detect if we finished the solution
        if s.group_pos == sr.groups.len() {
            s.extra_space -= 1;
        }
        Some(s)
    }
}

#[derive(Debug)]
struct SpringRecord {
    record: Vec<Condition>,
    groups: Vec<usize>,
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
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Ok(SpringRecord::new(record, groups))
    }
}

impl SpringRecord {
    fn new(record: Vec<Condition>, groups: Vec<usize>) -> SpringRecord {
        SpringRecord { record, groups }
    }

    fn multiply(&self, count: usize) -> SpringRecord {
        let record = (0..count)
            .map(|_| self.record.clone())
            .collect::<Vec<_>>()
            .join(&Condition::Unk);
        let groups = (0..count)
            .map(|_| self.groups.clone())
            .collect::<Vec<_>>()
            .concat();
        SpringRecord { record, groups }
    }

    fn count_solutions(&self) -> usize {
        self.count_solutions_inner(&mut Solution::new(self))
    }

    fn count_solutions_inner(&self, solution: &Solution) -> usize {
        if solution.complete(self) {
            return 1;
        }
        let mut count = 0;
        for n_spaces in 0..=solution.extra_space {
            if let Some(s) = solution.push(n_spaces, self) {
                count += self.count_solutions_inner(&s);
            }
        }
        return count;
    }
}

pub enum RunMode {
    Fast,
    Time,
}

pub fn run(input: &str, mode: RunMode) -> usize {
    match mode {
        RunMode::Fast => run_fast(input),
        RunMode::Time => run_time(input),
    }
}

pub fn run_fast(input: &str) -> usize {
    let records = input
        .lines()
        .map(|line| line.parse::<SpringRecord>().unwrap().multiply(5))
        .collect::<Vec<_>>();
    records
        .par_iter()
        .tqdm()
        .map(|sr| sr.count_solutions())
        .sum()
}

pub fn run_time(input: &str) -> usize {
    let records = input
        .lines()
        .map(|line| line.parse::<SpringRecord>().unwrap().multiply(5))
        .collect::<Vec<_>>();
    records
        .iter()
        .enumerate()
        .map(|(i, sr)| {
            let now = Instant::now();
            let count = sr.count_solutions();
            println!("{} took {}", i, now.elapsed().as_secs_f32());
            count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_12::b::RunMode;

    use super::SpringRecord;

    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, RunMode::Fast), 525152);
    }
    #[test]
    fn test_trivial_1() {
        assert_eq!("# 1".parse::<SpringRecord>().unwrap().count_solutions(), 1);
    }
    #[test]
    fn test_trivial_2() {
        assert_eq!("? 1".parse::<SpringRecord>().unwrap().count_solutions(), 1);
    }
    #[test]
    fn test_trivial_3() {
        assert_eq!(
            "#.# 1,1".parse::<SpringRecord>().unwrap().count_solutions(),
            1
        );
    }
    #[test]
    fn test_trivial_4() {
        assert_eq!(
            "??? 1,1".parse::<SpringRecord>().unwrap().count_solutions(),
            1
        );
    }
    #[test]
    fn test_trivial_5() {
        assert_eq!(
            "#??? 1,1"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            2
        );
    }
    #[test]
    fn test_trivial_6() {
        assert_eq!(
            "#???? 1,2"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            2
        );
    }
    #[test]
    fn test_trivial_7() {
        assert_eq!(
            "#???# 1,2"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            1
        );
    }

    #[test]
    fn test_trivial_8() {
        assert_eq!(
            "????? 1,1,1"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            1
        );
    }

    #[test]
    fn test_trivial_9() {
        assert_eq!(
            "?????? 1,1,1"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            4
        );
    }

    #[test]
    fn test_trivial_10() {
        assert_eq!(
            "?????? 1,2,1"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            1
        );
    }

    #[test]
    fn test_trivial_11() {
        // TODO debug this
        assert_eq!(
            "#.. 1"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            1
        );
    }

    #[test]
    fn test_a_line_1() {
        assert_eq!(
            "???.### 1,1,3"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            1
        );
    }
    #[test]
    fn test_a_line_2() {
        assert_eq!(
            ".??..??...?##. 1,1,3"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            4
        );
    }
    #[test]
    fn test_a_line_3() {
        assert_eq!(
            "?#?#?#?#?#?#?#? 1,3,1,6"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            1
        );
    }
    #[test]
    fn test_a_line_4() {
        // TODO: debug this
        assert_eq!(
            "????.#...#... 4,1,1"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(),
            1
        );
    }
    #[test]
    fn test_line_1() {
        assert_eq!(
            // will expand to 39 record len
            // solution requires 39 len
            // 0 extra space
            "???.### 1,1,3"
                .parse::<SpringRecord>()
                .unwrap()
                .multiply(5)
                .count_solutions(),
            1
        );
    }
    #[test]
    fn test_line_2() {
        assert_eq!(
            ".??..??...?##. 1,1,3"
                .parse::<SpringRecord>()
                .unwrap()
                .multiply(5)
                .count_solutions(),
            16384
        );
    }
    #[test]
    fn test_line_3() {
        assert_eq!(
            "?#?#?#?#?#?#?#? 1,3,1,6"
                .parse::<SpringRecord>()
                .unwrap()
                .multiply(5)
                .count_solutions(),
            1
        );
    }
    #[test]
    fn test_line_4() {
        assert_eq!(
            "????.#...#... 4,1,1"
                .parse::<SpringRecord>()
                .unwrap()
                .multiply(5)
                .count_solutions(),
            16
        );
    }
    #[test]
    fn test_line_5() {
        assert_eq!(
            "????.######..#####. 1,6,5"
                .parse::<SpringRecord>()
                .unwrap()
                .multiply(5)
                .count_solutions(),
            2500
        );
    }
    #[test]
    fn test_line_6() {
        assert_eq!(
            "?###???????? 3,2,1"
                .parse::<SpringRecord>()
                .unwrap()
                .multiply(5)
                .count_solutions(),
            506250
        );
    }
    // #[test]
    // fn slow_test_5() {
    //     "????????.?##???????? 3,1,1,4,1,1"
    //         .parse::<SpringRecord>()
    //         .unwrap()
    //         .multiply(5)
    //         .count_solutions();
    // }
}
