use std::str::FromStr;

#[derive(Hash)]
enum Pix {
    Roc,
    Ash,
}

impl TryFrom<char> for Pix {
    type Error = &'static str;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Pix::Ash,
            '#' => Pix::Roc,
            _ => "Invalid char",
        }
    }
}

struct Pattern {
    pix: Vec<Vec<Pix>>,
}

struct Patterns {
    patterns: Vec<Patter>,
}

impl FromStr for Patters {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut patterns = Vec::new();
        let lines = Vec::new();
        for line in s.lines() {
            if line.len() > 0 {
                lines.push(line);
            } else {
                patterns.push(lines.join("\n").parse<Patter>().unwrap());
            }
        }
        Ok(Patters{patterns})
    }
}

pub fn run(_input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 0);
    }
}
