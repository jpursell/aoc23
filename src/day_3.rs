pub fn day_3() {
    let input = include_str!("day_3_data.txt");
    println!("day 3a {}", day_3a(input));
}

struct Schematic<'a> {
    lines: Vec<&'a str>,
    width: usize,
}

fn is_op(c: char) -> bool {
    match c {
        '*' => true,
        '+' => true,
        '#' => true,
        '$' => true,
        '-' => true,
        '/' => true,
        '=' => true,
        '%' => true,
        '&' => true,
        '@' => true,
        '.' => false,
        other => {
            if other.is_numeric() {
                false
            } else {
                panic!("Unexpected char {}", c)
            }
        }
    }
}

enum State {
    Looking,
    Found,
}

struct NumLocation {
    iline: usize,
    start: usize,
    end: usize,
}

impl NumLocation {
    fn new(iline: usize, start: usize, end: usize) -> NumLocation {
        NumLocation { iline, start, end }
    }
}

impl<'a> Schematic<'a> {
    fn new(input: &str) -> Schematic {
        let lines = input
            .split("\n")
            .map(|line| line.trim())
            .collect::<Vec<&str>>();
        let width = lines[0].len();
        for line in &lines {
            assert_eq!(width, line.len());
        }
        Schematic { lines, width }
    }

    fn extract_part(&self, loc: &NumLocation) -> u32 {
        self.lines[loc.iline][loc.start..loc.end]
            .parse::<u32>()
            .unwrap()
    }

    fn is_part(&self, loc: &NumLocation) -> bool {
        // let num = self.lines[iline][start..end].parse::<u32>().unwrap();
        let left = if loc.start == 0 {
            0
        } else {
            if is_op(self.lines[loc.iline].chars().nth(loc.start - 1).unwrap()) {
                return true;
            }
            loc.start - 1
        };
        let right = if loc.end == self.width {
            self.width
        } else {
            if is_op(self.lines[loc.iline].chars().nth(loc.end).unwrap()) {
                return true;
            }
            loc.end + 1
        };
        // check top
        if loc.iline > 0 {
            for c in self.lines[loc.iline - 1][left..right].chars() {
                if is_op(c) {
                    return true;
                }
            }
        }
        // check bottom
        if loc.iline < self.lines.len() - 1 {
            for c in self.lines[loc.iline + 1][left..right].chars() {
                if is_op(c) {
                    return true;
                }
            }
        }
        false
    }

    fn find_num_locations(&self) -> Vec<NumLocation> {
        let mut output = Vec::new();
        for (iline, line) in self.lines.iter().enumerate() {
            let mut start = 0;
            let mut state = State::Looking;
            for (pos, c) in line.chars().enumerate() {
                match state {
                    State::Looking => {
                        if c.is_numeric() {
                            start = pos;
                            state = State::Found;
                        }
                    }
                    State::Found => {
                        if !c.is_numeric() {
                            state = State::Looking;
                            output.push(NumLocation::new(iline, start, pos));
                        }
                    }
                }
            }
            match state {
                State::Found => {
                    output.push(NumLocation::new(iline, start, line.len()));
                }
                _ => (),
            }
        }
        output
    }
}

fn day_3a(input: &str) -> u32 {
    let schematic = Schematic::new(input);
    let parts = schematic
        .find_num_locations()
        .into_iter()
        .filter(|loc| schematic.is_part(loc))
        .collect::<Vec<NumLocation>>();
    parts.iter().map(|loc| schematic.extract_part(loc)).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(super::day_3a(input), 4361);
    }
}
