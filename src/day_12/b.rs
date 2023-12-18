use rayon::prelude::*;
use simple_tqdm::ParTqdm;
use std::{str::FromStr, time::Instant};

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
struct Solution<'a> {
    solution: Vec<Condition>,
    /// Location of next unknown_pos to add
    pos: usize,
    /// Location of ? in record.record
    unknown_pos: Vec<usize>,
    /// index of next group to add
    group_pos: Option<usize>,
    pos_hist: Vec<usize>,
    record: &'a SpringRecord,
}

impl<'a> Solution<'a> {
    /// Return the index of the next group to add
    fn find_group_pos(solution: &Vec<Condition>) -> Option<usize> {
        let mut count = 0;
        for cluster in solution.split(|&c| c == Condition::Operational) {
            if cluster.len() == 0 {
                continue;
            }
            if cluster.iter().any(|&c| c == Condition::Unknown) {
                return Some(count);
            } else {
                count += 1;
            }
        }
        None
    }

    fn new(sr: &SpringRecord) -> Solution {
        let solution = sr.record.clone();
        let unknown_pos = solution
            .iter()
            .enumerate()
            .filter(|(_, &c)| c == Condition::Unknown)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        let group_pos = Solution::find_group_pos(&solution);
        Solution {
            solution,
            pos: 0,
            unknown_pos,
            pos_hist: Vec::new(),
            record: sr,
            group_pos,
        }
    }

    fn complete(&self) -> bool {
        self.pos >= self.unknown_pos.len()
    }

    fn push(&mut self, c: &Condition) -> Result<(), ()> {
        match c {
            Condition::Damaged => {
                if self.group_pos.unwrap() >= self.record.groups.len() {
                    return Err(());
                }
                let current_group_size = self.record.groups[self.group_pos.unwrap()];
                let insert_pos = self.unknown_pos[self.pos];

                let group_size = self.solution[0..insert_pos]
                    .rsplit(|c| *c != Condition::Damaged)
                    .next()
                    .unwrap()
                    .len();
                if group_size + 1 > current_group_size {
                    return Err(());
                }
                let max_group_size = self.solution[insert_pos..]
                    .split(|c| *c == Condition::Operational)
                    .next()
                    .unwrap()
                    .len()
                    + group_size;
                if max_group_size < current_group_size {
                    return Err(());
                }
                let mut n_inserted = 0;
                let mut pattern = vec![Condition::Damaged].repeat(current_group_size - group_size);
                pattern.push(Condition::Operational);

                for p in pattern.iter() {
                    self.solution[insert_pos + n_inserted] = *p;
                    n_inserted += 1;
                    if insert_pos + n_inserted >= self.solution.len()
                        || self.solution[insert_pos + n_inserted] != Condition::Unknown
                    {
                        break;
                    }
                }
                self.pos_hist.push(self.pos);
                self.pos += n_inserted;
                self.group_pos = Solution::find_group_pos(&self.solution);
                return Ok(());
            }
            Condition::Operational => {
                // If there are no more groups just fill with .
                if self.group_pos.unwrap() >= self.record.groups.len() {
                    self.pos_hist.push(self.pos);
                    while !self.complete() {
                        self.solution[self.unknown_pos[self.pos]] = Condition::Operational;
                        self.pos += 1;
                    }
                    self.group_pos = Solution::find_group_pos(&self.solution);
                    return Ok(());
                }

                let current_group_size = self.record.groups[self.group_pos.unwrap()];
                let insert_pos = self.unknown_pos[self.pos];

                let group_size = self.solution[0..insert_pos]
                    .rsplit(|c| *c != Condition::Damaged)
                    .next()
                    .unwrap()
                    .len();
                if group_size > 0 {
                    if group_size != current_group_size {
                        return Err(());
                    } else {
                        self.pos_hist.push(self.pos);
                        self.solution[self.unknown_pos[self.pos]] = Condition::Operational;
                        self.pos += 1;
                        self.group_pos = Solution::find_group_pos(&self.solution);
                        return Ok(());
                    }
                }

                if self.pos == self.unknown_pos.len() - 1
                    || self.unknown_pos[self.pos + 1] != insert_pos + 1
                {
                    // we are not in a cluster situation so just insert . and go
                    self.solution[self.unknown_pos[self.pos]] = Condition::Operational;
                    self.pos_hist.push(self.pos);
                    self.pos += 1;
                    self.group_pos = Solution::find_group_pos(&self.solution);
                    return Ok(());
                } else {
                    // we are in a cluster of .?? so check how many
                    let cluster_size = self.solution[insert_pos..]
                        .split(|c| *c != Condition::Unknown)
                        .next()
                        .unwrap()
                        .len();
                    if cluster_size <= current_group_size {
                        // fill it up
                        self.pos_hist.push(self.pos);
                        for _ in 0..cluster_size {
                            self.solution[self.unknown_pos[self.pos]] = Condition::Operational;
                            self.pos += 1;
                        }
                        self.group_pos = Solution::find_group_pos(&self.solution);
                        return Ok(());
                    } else {
                        // just insert a single .
                        self.pos_hist.push(self.pos);
                        self.solution[self.unknown_pos[self.pos]] = Condition::Operational;
                        self.pos += 1;
                        self.group_pos = Solution::find_group_pos(&self.solution);
                        return Ok(());
                    }
                }
            }
            _ => panic!(),
        }
    }

    fn pop(&mut self) {
        while self.pos != *self.pos_hist.last().unwrap() {
            self.pos -= 1;
            self.solution[self.unknown_pos[self.pos]] = Condition::Unknown;
        }
        self.pos_hist.pop();
        self.group_pos = Solution::find_group_pos(&self.solution);
    }

    /// Look at first groups of Condition::Damaged and see if they match
    fn check_first_groups(&self) -> bool {
        let no_unknown = self.complete();
        let first = if no_unknown {
            &self.solution[..]
        } else {
            let mut first = &self.solution[..self.unknown_pos[self.pos]];
            while first.last() == Some(&Condition::Damaged) {
                first = &first[..first.len() - 1];
            }
            first
        };
        let first_groups = first
            .split(|&c| c == Condition::Operational)
            .map(|c| c.len())
            .filter(|&n| n > 0)
            .collect::<Vec<_>>();
        if self
            .record
            .groups
            .iter()
            .zip(first_groups.iter())
            .any(|(&g0, &g1)| g0 != g1)
        {
            return false;
        }
        if no_unknown {
            return true;
        }
        let mut second = &self.solution[first.len()..];
        while second.first() == Some(&Condition::Operational) {
            second = &second[1..];
        }
        let second = second
            .split(|&c| c == Condition::Operational)
            .next()
            .unwrap();
        if first_groups.len() >= self.record.groups.len() {
            // all groups used up in first group
            let second = &self.solution[first.len()..];
            if second.iter().any(|&c| c == Condition::Damaged) {
                return false;
            } else {
                return true;
            }
        }
        let second_group = self.record.groups[first_groups.len()];
        {
            let min_second_size = second
                .split(|&c| c == Condition::Unknown)
                .next()
                .unwrap()
                .len();
            if min_second_size > second_group {
                return false;
            }
        }
        {
            // For checking the max next group size we need to check for this
            // ..???..?#?... but not further then the next group with a
            // '#' in it
            let mut second = &self.solution[first.len()..];
            while second.first() == Some(&Condition::Operational) {
                second = &second[1..];
            }

            let mut max_second_size = 0;
            for group in second.split(|&c| c == Condition::Operational) {
                max_second_size = max_second_size.max(group.len());
                if group.iter().any(|&c| c == Condition::Damaged) {
                    break;
                }
            }

            if max_second_size < second_group {
                return false;
            }
        }
        true
    }

    /// Look at number of possible groups of Condition::Damaged and see if they match
    fn check_num_groups(&self) -> bool {
        if self.complete() {
            return self
                .solution
                .split(|&c| c == Condition::Operational)
                .map(|c| c.len())
                .filter(|&n| n > 0)
                .count()
                == self.record.groups.len();
        }

        assert!(self.solution.len() > 0);

        {
            let mut max_group_solution = self.solution.clone();
            if max_group_solution[0] == Condition::Unknown {
                max_group_solution[0] = Condition::Damaged;
            }
            (1..max_group_solution.len()).for_each(|i| {
                if max_group_solution[i] != Condition::Unknown {
                    return;
                }
                match max_group_solution[i - 1] {
                    Condition::Damaged => {
                        max_group_solution[i] = Condition::Operational;
                    }
                    Condition::Operational => {
                        max_group_solution[i] = Condition::Damaged;
                    }
                    Condition::Unknown => panic!(),
                }
            });
            let max_groups = max_group_solution
                .split(|&c| c == Condition::Operational)
                .map(|c| c.len())
                .filter(|&n| n > 0)
                .count();
            if max_groups < self.record.groups.len() {
                return false;
            }
        }
        {
            let mut min_group_solution = self.solution.clone();
            if min_group_solution[0] == Condition::Unknown {
                min_group_solution[0] = Condition::Operational;
            }
            (1..min_group_solution.len()).for_each(|i| {
                if min_group_solution[i] != Condition::Unknown {
                    return;
                }
                match min_group_solution[i - 1] {
                    Condition::Damaged => {
                        min_group_solution[i] = Condition::Damaged;
                    }
                    Condition::Operational => {
                        min_group_solution[i] = Condition::Operational;
                    }
                    Condition::Unknown => panic!(),
                }
            });
            let min_groups = min_group_solution
                .split(|&c| c == Condition::Operational)
                .map(|c| c.len())
                .filter(|&n| n > 0)
                .count();
            if min_groups > self.record.groups.len() {
                return false;
            }
        }

        true
    }

    /// Check to see if current solution is possible. Can handle unknowns.
    ///
    /// The answer should be saved in a bool
    fn check(&self) -> bool {
        if !self.check_first_groups() {
            return false;
        }
        if !self.check_num_groups() {
            return false;
        }
        true
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

// fn format_record(r: &Vec<Condition>) -> String {
//     r.iter()
//         .map(|&c| match c {
//             Condition::Damaged => '#',
//             Condition::Operational => '.',
//             Condition::Unknown => '?',
//         })
//         .collect::<String>()
// }

// fn format_groups(g: &Vec<usize>) -> String {
//     g.iter().map(|&g| format!("{}", g)).join(",")
// }

impl SpringRecord {
    fn new(record: Vec<Condition>, groups: Vec<usize>) -> SpringRecord {
        SpringRecord { record, groups }
    }

    fn multiply(&self, count: usize) -> SpringRecord {
        let record = (0..count)
            .map(|_| self.record.clone())
            .collect::<Vec<_>>()
            .join(&Condition::Unknown);
        let groups = (0..count)
            .map(|_| self.groups.clone())
            .collect::<Vec<_>>()
            .concat();
        SpringRecord { record, groups }
    }

    fn count_solutions(&self) -> usize {
        self.count_solutions_inner(&mut Solution::new(self))
    }

    fn count_solutions_inner(&self, solution: &mut Solution) -> usize {
        if !solution.check() {
            return 0;
        }

        if solution.complete() {
            return 1;
        } else {
            let mut count = 0;
            if let Ok(_) = solution.push(&Condition::Damaged) {
                count += self.count_solutions_inner(solution);
                solution.pop();
            }

            if let Ok(_) = solution.push(&Condition::Operational) {
                count += self.count_solutions_inner(solution);
                solution.pop();
            }

            return count;
        }
    }
}

pub enum RunMode {
    Fast,
    Time,
}

pub fn run(input: &str, mode:RunMode) -> usize {
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
            println!("{} took {}",i, now.elapsed().as_secs_f32());
            count
        }
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day_12::b::{Condition, Solution, RunMode};

    use super::SpringRecord;

    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input, RunMode::Fast), 525152);
    }
    #[test]
    fn test_group_pos_1() {
        let sr = ".#?# 1,1".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert_eq!(s.group_pos, Some(0));
    }
    #[test]
    fn test_group_pos_2() {
        let sr = ".#.?# 1,1".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert_eq!(s.group_pos, Some(1));
    }
    #[test]
    fn test_group_pos_3() {
        let sr = ".#..# 1,1".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert_eq!(s.group_pos, None);
    }
    #[test]
    fn test_check_1() {
        let sr = ".#. 1".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(s.check())
    }
    #[test]
    fn test_check_1a() {
        let sr = "... 1".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(!s.check())
    }
    #[test]
    fn test_check_1b() {
        let sr = ".#. 2".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(!s.check())
    }
    #[test]
    fn test_check_1c() {
        let sr = ".#.##? 1,2".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(s.check())
    }
    #[test]
    fn test_check_1ca() {
        let sr = ".#.##? 1,1".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(!s.check())
    }
    #[test]
    fn test_check_1cb() {
        let sr = ".#.##? 1,4".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(!s.check())
    }
    #[test]
    fn test_check_1d() {
        let sr = ".#.## 1,2".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(s.check())
    }
    #[test]
    fn test_check_1e() {
        let sr = ".#.## 1,3".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(!s.check())
    }
    #[test]
    fn test_check_1f() {
        let sr = ".#??# 1,1,1".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(!s.check())
    }
    #[test]
    fn test_check_1fa() {
        let sr = ".#??# 5".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(!s.check())
    }
    #[test]
    fn test_check_1ga() {
        let sr = ".#??#.# 4,1".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(s.check())
    }
    #[test]
    fn test_check_1g() {
        let sr = ".#??#.# 3".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(!s.check())
    }
    #[test]
    fn test_check_2() {
        let sr = ".?. 1".parse::<SpringRecord>().unwrap();
        let s = Solution::new(&sr);
        assert!(s.check())
    }
    #[test]
    fn test_check_3() {
        let sr = ".?. 1".parse::<SpringRecord>().unwrap();
        let mut s = Solution::new(&sr);
        s.push(&Condition::Damaged).unwrap();
        assert!(s.check())
    }
    #[test]
    fn test_check_4() {
        let sr = ".?. 1".parse::<SpringRecord>().unwrap();
        let mut s = Solution::new(&sr);
        s.push(&Condition::Operational).unwrap();
        assert!(!s.check())
    }
    #[test]
    fn test_line_1() {
        assert_eq!(
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
}
