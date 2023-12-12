use std::str::FromStr;

#[derive(Debug)]
struct Sequence {
    numbers: Vec<i64>,
}

impl FromStr for Sequence {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Self, <Self as FromStr>::Err> {
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        Ok(Sequence { numbers })
    }
}
impl Sequence {
    fn predict_prev(&self) -> i64 {
        let mut vecs = Vec::new();
        vecs.push(self.numbers.clone());
        while vecs.last().unwrap().iter().any(|x| *x != 0) {
            let next = vecs
                .last()
                .unwrap()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect::<Vec<i64>>();
            vecs.push(next);
        }
        let mut val = 0;
        for vec in vecs.iter().rev().skip(1) {
            val = vec[0] - val
        }
        val
    }
}

pub fn run(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.parse::<Sequence>().unwrap().predict_prev())
        .sum::<i64>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 2);
    }
}
