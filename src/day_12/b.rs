use std::{str::FromStr, time::Instant};

use itertools::Itertools;

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
    groups: Vec<usize>,
    num_unknown: usize,
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

fn format_record(r: &Vec<Condition>) -> String {
    r.iter()
        .map(|&c| match c {
            Condition::Damaged => '#',
            Condition::Operational => '.',
            Condition::Unknown => '?',
        })
        .collect::<String>()
}

fn format_groups(g: &Vec<usize>) -> String {
    g.iter().map(|&g| format!("{}", g)).join(",")
}

impl SpringRecord {
    fn new(record: Vec<Condition>, groups: Vec<usize>) -> SpringRecord {
        let num_unknown = record.iter().filter(|&&c| c == Condition::Unknown).count();
        SpringRecord {
            record,
            groups,
            num_unknown,
        }
    }
    /// Check if there are no more unknowns
    fn complete(&self, solution: &Vec<Condition>) -> bool {
        self.num_unknown == solution.len()
    }

    fn combine_solution(&self, solution: &Vec<Condition>) -> Vec<Condition> {
        let mut sol_chars = solution.iter();
        self.record
            .iter()
            .map(|&c| {
                if c == Condition::Unknown {
                    if let Some(&s) = sol_chars.next() {
                        return s;
                    } else {
                        return Condition::Unknown;
                    }
                } else {
                    return c;
                }
            })
            .collect::<Vec<_>>()
    }

    /// Look at first groups of Condition::Damaged and see if they match
    fn check_first_groups(&self, solution: &Vec<Condition>) -> bool {
        let no_unknown = solution.iter().all(|&c| c != Condition::Unknown);
        let first = if no_unknown {
            &solution[..]
        } else {
            let mut first = solution.split(|&c| c == Condition::Unknown).next().unwrap();
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
            .groups
            .iter()
            .zip(first_groups.iter())
            .any(|(&g0, &g1)| g0 != g1)
        {
            // println!("r {}",format_record(&self.record));
            // println!("s {}",format_record(solution));
            // println!("expected {}",format_groups(&self.groups));
            // println!("but got  {}",format_groups(&first_groups));
            return false;
        }
        if no_unknown {
            return true;
        }
        let mut second = &solution[first.len()..];
        while second.first() == Some(&Condition::Operational) {
            second = &second[1..];
        }
        let second = second
            .split(|&c| c == Condition::Operational)
            .next()
            .unwrap();
        if first_groups.len() >= self.groups.len() {
            // all groups used up in first group
            let second = &solution[first.len()..];
            if second.iter().any(|&c| c == Condition::Damaged) {
                // println!("r {}", format_record(&self.record));
                // println!("s {}", format_record(solution));
                // println!("expected {}", format_groups(&self.groups));
                // println!("and got  {}", format_groups(&first_groups));
                return false;
            } else {
                return true;
            }
        }
        let second_group = self.groups[first_groups.len()];
        {
            let min_second_size = second
                .split(|&c| c == Condition::Unknown)
                .next()
                .unwrap()
                .len();
            if min_second_size > second_group {
                // TODO make a good debug message
                println!("r {}", format_record(&self.record));
                println!("s {}", format_record(solution));
                println!("expected {}", format_groups(&self.groups));
                println!("but got  {}", format_groups(&first_groups));
                return false;
            }
        }
        {
            let max_second_size = second.len();
            if max_second_size < second_group {
                return false;
            }
        }
        true
    }

    /// Look at number of possible groups of Condition::Damaged and see if they match
    fn check_num_groups(&self, solution: &Vec<Condition>) -> bool {
        if solution.iter().all(|&c| c != Condition::Unknown) {
            return solution
                .split(|&c| c == Condition::Operational)
                .map(|c| c.len())
                .filter(|&n| n > 0)
                .count()
                == self.groups.len();
        }

        assert!(solution.len() > 0);

        {
            let mut max_group_solution = solution.clone();
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
            if max_groups < self.groups.len() {
                return false;
            }
        }
        {
            let mut min_group_solution = solution.clone();
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
            if min_groups > self.groups.len() {
                return false;
            }
        }

        true
    }

    /// Check to see if current solution is possible. Can handle unknowns.
    ///
    /// The answer should be saved in a bool
    fn check(&self, solution: &Vec<Condition>) -> bool {
        let solution = self.combine_solution(solution);
        if !self.check_first_groups(&solution) {
            return false;
        }
        if !self.check_num_groups(&solution) {
            return false;
        }
        true
    }

    fn append_solution(&self, solution: &mut Vec<Condition>, condition: &Condition) {
        solution.push(*condition);
    }

    fn undo_solution(&self, solution: &mut Vec<Condition>) {
        solution.pop();
    }

    fn count_solutions(&self, solution: &mut Vec<Condition>) -> usize {
        if !self.check(solution) {
            return 0;
        }

        if self.complete(&solution) {
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
        count += line
            .parse::<SpringRecord>()
            .unwrap()
            .count_solutions(&mut Vec::new());
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
    use crate::day_12::b::Condition;

    use super::SpringRecord;

    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 525125);
    }
    #[test]
    fn test_check_1() {
        assert!(SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Operational
            ],
            vec![1]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1a() {
        assert!(!SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Operational,
                Condition::Operational
            ],
            vec![1]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1b() {
        assert!(!SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Operational
            ],
            vec![2]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1c() {
        assert!(SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged,
                Condition::Damaged,
                Condition::Unknown,
            ],
            vec![1, 2]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1ca() {
        assert!(!SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged,
                Condition::Damaged,
                Condition::Unknown,
            ],
            vec![1, 1]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1cb() {
        assert!(!SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged,
                Condition::Damaged,
                Condition::Unknown,
            ],
            vec![1, 4]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1d() {
        assert!(SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged,
                Condition::Damaged
            ],
            vec![1, 2]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1e() {
        assert!(!SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged,
                Condition::Damaged
            ],
            vec![1, 3]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1f() {
        assert!(!SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Damaged
            ],
            vec![1, 1, 1]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1fa() {
        assert!(!SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Damaged
            ],
            vec![5]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1ga() {
        assert!(SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged
            ],
            vec![4, 1]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_1g() {
        assert!(!SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Damaged,
                Condition::Unknown,
                Condition::Unknown,
                Condition::Damaged,
                Condition::Operational,
                Condition::Damaged
            ],
            vec![3]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_2() {
        assert!(SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Unknown,
                Condition::Operational
            ],
            vec![1]
        )
        .check(&Vec::new()));
    }
    #[test]
    fn test_check_3() {
        assert!(SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Unknown,
                Condition::Operational
            ],
            vec![1]
        )
        .check(&vec![Condition::Damaged]));
    }
    #[test]
    fn test_check_4() {
        assert!(!SpringRecord::new(
            vec![
                Condition::Operational,
                Condition::Unknown,
                Condition::Operational
            ],
            vec![1]
        )
        .check(&vec![Condition::Operational]));
    }
    #[test]
    fn test_line_1() {
        assert_eq!(
            "???.### 1,1,3"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(&mut Vec::new()),
            1
        );
    }
    #[test]
    fn test_line_2() {
        assert_eq!(
            ".??..??...?##. 1,1,3"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(&mut Vec::new()),
            16384
        );
    }
    #[test]
    fn test_line_3() {
        assert_eq!(
            "?#?#?#?#?#?#?#? 1,3,1,6"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(&mut Vec::new()),
            1
        );
    }
    #[test]
    fn test_line_4() {
        assert_eq!(
            "????.#...#... 4,1,1"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(&mut Vec::new()),
            16
        );
    }
    #[test]
    fn test_line_5() {
        assert_eq!(
            "????.######..#####. 1,6,5"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(&mut Vec::new()),
            2500
        );
    }
    #[test]
    fn test_line_6() {
        assert_eq!(
            "?###???????? 3,2,1"
                .parse::<SpringRecord>()
                .unwrap()
                .count_solutions(&mut Vec::new()),
            506250
        );
    }
}
