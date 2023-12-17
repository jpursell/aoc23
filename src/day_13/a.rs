use std::str::FromStr;

#[derive(Debug, Hash)]
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

#[derive(Debug)]
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
    fn summarize(&self) -> usize {
        dbg!(self);
        todo!()
    }
}

#[derive(Debug)]
struct Patterns {
    patterns: Vec<Pattern>,
}

impl Patterns {
    fn summarize(&self) -> usize {
        self.patterns.iter().map(|p| p.summarize()).sum()
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
            }
        }
        Ok(Patterns { patterns })
    }
}

pub fn run(input: &str) -> usize {
    input.parse::<Patterns>().unwrap().summarize()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 405);
    }
}
