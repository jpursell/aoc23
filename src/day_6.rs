use std::str::FromStr;

pub fn day_6() {
    // let input = include_str!("day_6_data.txt");
    // println!("day n a {}", day_6_a(input));
    // println!("day n b {}", day_6_b(input));
}

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

#[derive(Debug)]
struct RaceHistory {
    races: Vec<Race>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRaceHistoryError;

impl FromStr for RaceHistory {
    type Err = ParseRaceHistoryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        assert_eq!(lines.len(), 2);

        let (_, times) = lines[0].split_once("Time:").unwrap();
        let times = times
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let (_, distances) = lines[1].split_once("Distance:").unwrap();
        let distances = distances
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        assert_eq!(times.len(), distances.len());

        let races = times
            .iter()
            .zip(distances.iter())
            .map(|(t, d)| Race {
                time: *t,
                distance: *d,
            })
            .collect::<Vec<_>>();

        Ok(RaceHistory { races })
    }
}

impl RaceHistory {
    fn num_ways_to_win(&self) -> u64 {
        0
    }
}

fn day_6_a(input: &str) -> u64 {
    dbg!(input
        .parse::<RaceHistory>()
        .expect("Failed to parse RaceHistory"))
    .num_ways_to_win()
}

fn day_6_b(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;
        assert_eq!(super::day_6_a(input), 288);
        assert_eq!(super::day_6_b(input), 0);
    }
}
