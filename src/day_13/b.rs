use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str::FromStr,
};

#[derive(Debug, Copy, Clone, Hash)]
enum Pix {
    Roc,
    Ash,
}

impl TryFrom<char> for Pix {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Pix::Ash),
            '#' => Ok(Pix::Roc),
            _ => Err("Invalid char"),
        }
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    pix: Vec<Vec<Pix>>,
}

impl FromStr for Pattern {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pattern {
            pix: s
                .lines()
                .map(|line| {
                    line.chars()
                        .map(|c| Pix::try_from(c).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        })
    }
}

impl Pattern {
    fn find_mirror(seq: &Vec<u64>, skip: Option<usize>) -> Option<usize> {
        let starts = seq
            .windows(2)
            .enumerate()
            .filter(|(i, v)| {
                if let Some(s) = skip {
                    if s == (*i) + 1 {
                        return false;
                    }
                }
                v[0] == v[1]
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for start in starts {
            let all_match = seq[start + 1..]
                .iter()
                .zip(seq[..start + 1].iter().rev())
                .all(|(a, b)| a == b);
            if all_match {
                return Some(start + 1);
            }
        }
        None
    }
    fn hash_rows(&self) -> Vec<u64> {
        self.pix
            .iter()
            .map(|row| {
                let mut s = DefaultHasher::new();
                row.iter().for_each(|p| {
                    p.hash(&mut s);
                });
                s.finish()
            })
            .collect::<Vec<_>>()
    }
    fn hash_cols(&self) -> Vec<u64> {
        let mut hashers = self.pix[0]
            .iter()
            .map(|_| DefaultHasher::new())
            .collect::<Vec<_>>();
        self.pix.iter().for_each(|row| {
            row.iter()
                .zip(hashers.iter_mut())
                .for_each(|(p, s)| p.hash(s))
        });
        hashers.iter().map(|s| s.finish()).collect::<Vec<_>>()
    }

    fn swap(&mut self, irow: usize, icol: usize) {
        let col = &mut self.pix[irow][icol];
        *col = match *col {
            Pix::Ash => Pix::Roc,
            Pix::Roc => Pix::Ash,
        };
    }

    fn summarize(&self) -> usize {
        if let Some(row) = Pattern::find_mirror(&self.hash_rows(), None) {
            return row * 100;
        };
        if let Some(col) = Pattern::find_mirror(&self.hash_cols(), None) {
            return col;
        };
        panic!()
    }

    fn summarize_smudge(&self) -> usize {
        let s = self.summarize();
        for (irow, row) in self.pix.iter().enumerate() {
            for (icol, _col) in row.iter().enumerate() {
                let c = &mut self.clone();
                c.swap(irow, icol);
                if let Some(row) = Pattern::find_mirror(&c.hash_rows(), Some(s / 100)) {
                    let val = row * 100;
                    // println!("irow {} icol {} row {} val {}", irow, icol, row, val);
                    if val != s {
                        return val;
                    }
                };
                if let Some(col) = Pattern::find_mirror(&c.hash_cols(), Some(s)) {
                    // println!("irow {} icol {} col {}", irow, icol, col);
                    if col != s {
                        return col;
                    }
                };
            }
        }
        panic!()
    }
}

#[derive(Debug)]
struct Patterns {
    patterns: Vec<Pattern>,
}

impl Patterns {
    fn summarize(&self) -> usize {
        self.patterns
            .iter()
            .enumerate()
            // .inspect(|(i, _)| println!("process map {}", i))
            .map(|(_, p)| p.summarize_smudge())
            .sum()
    }
}

impl FromStr for Patterns {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut patterns = Vec::new();
        let mut lines = Vec::new();
        for line in s.lines() {
            if line.len() > 0 {
                lines.push(line);
            } else {
                patterns.push(lines.join("\n").parse::<Pattern>().unwrap());
                lines.clear()
            }
        }
        patterns.push(lines.join("\n").parse::<Pattern>().unwrap());
        Ok(Patterns { patterns })
    }
}

pub fn run(input: &str) -> usize {
    input.parse::<Patterns>().unwrap().summarize()
}

#[cfg(test)]
mod tests {
    use super::Pattern;

    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 400);
    }
    #[test]
    fn trouble_case() {
        let input = r#"......########...
....#..#.##.#..#.
##....########...
###..#.#.##.#.#..
...#....####....#
#####.########.##
..#....##..##....
###....#.##.#....
##.#.#..#..#..#.#
###.....####.....
###.#.##....##.#.
###.#####..#####.
...#...#.##.#...#
..#####......####
..#.####.##.####.
......#.#.##.#...
..#.####.##.####."#;
        let p = input.parse::<Pattern>().unwrap();
        println!("summarize {}", p.summarize());
        println!("summarize smudge {}", p.summarize_smudge());
    }
}
