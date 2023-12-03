pub fn day_1() {
    let day_1_data = include_str!("day_1_data.txt");
    println!("day 1a {}", day_1a(day_1_data));
    println!("day 1b {}", day_1b(day_1_data));
}

pub fn day_1a(input: &str) -> u32 {
    fn find_first(line: &str) -> char {
        for c in line.chars() {
            if c.is_numeric() {
                return c;
            }
        }
        panic!("Did not find char");
    }
    fn find_last(line: &str) -> char {
        for c in line.chars().rev() {
            if c.is_numeric() {
                return c;
            }
        }
        panic!("Did not find char");
    }
    let mut sum: u32 = 0;
    for line in input.split("\n") {
        let first_digit = find_first(line);
        let last_digit = find_last(line);
        let num = format!("{}{}", first_digit, last_digit)
            .parse::<u32>()
            .unwrap();
        sum += num;
    }
    sum
}

pub fn day_1b(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.split("\n") {
        for (pos, char) in line.chars().enumerate() {
            match char {
                '1' => {
                    sum += 10;
                    break;
                }
                '2' => {
                    sum += 20;
                    break;
                }
                '3' => {
                    sum += 30;
                    break;
                }
                '4' => {
                    sum += 40;
                    break;
                }
                '5' => {
                    sum += 50;
                    break;
                }
                '6' => {
                    sum += 60;
                    break;
                }
                '7' => {
                    sum += 70;
                    break;
                }
                '8' => {
                    sum += 80;
                    break;
                }
                '9' => {
                    sum += 90;
                    break;
                }
                'o' => {
                    if line.len() - pos >= 3 && &line[pos + 1..pos + 3] == "ne" {
                        sum += 10;
                        break;
                    }
                }
                't' => {
                    if line.len() - pos >= 3 && &line[pos + 1..pos + 3] == "wo" {
                        sum += 20;
                        break;
                    }
                    if line.len() - pos >= 5 && &line[pos + 1..pos + 5] == "hree" {
                        sum += 30;
                        break;
                    }
                }
                'f' => {
                    if line.len() - pos >= 4 && &line[pos + 1..pos + 4] == "our" {
                        sum += 40;
                        break;
                    }
                    if line.len() - pos >= 4 && &line[pos + 1..pos + 4] == "ive" {
                        sum += 50;
                        break;
                    }
                }
                's' => {
                    if line.len() - pos >= 3 && &line[pos + 1..pos + 3] == "ix" {
                        sum += 60;
                        break;
                    }
                    if line.len() - pos >= 5 && &line[pos + 1..pos + 5] == "even" {
                        sum += 70;
                        break;
                    }
                }
                'e' => {
                    if line.len() - pos >= 5 && &line[pos + 1..pos + 5] == "ight" {
                        sum += 80;
                        break;
                    }
                }
                'n' => {
                    if line.len() - pos >= 4 && &line[pos + 1..pos + 4] == "ine" {
                        sum += 90;
                        break;
                    }
                }
                _ => (),
            }
        }
        let line = line.chars().rev().collect::<String>();
        for (pos, char) in line.chars().enumerate() {
            match char {
                '1' => {
                    sum += 1;
                    break;
                }
                '2' => {
                    sum += 2;
                    break;
                }
                '3' => {
                    sum += 3;
                    break;
                }
                '4' => {
                    sum += 4;
                    break;
                }
                '5' => {
                    sum += 5;
                    break;
                }
                '6' => {
                    sum += 6;
                    break;
                }
                '7' => {
                    sum += 7;
                    break;
                }
                '8' => {
                    sum += 8;
                    break;
                }
                '9' => {
                    sum += 9;
                    break;
                }
                'e' => {
                    if line.len() - pos >= 3 && &line[pos + 1..pos + 3] == "no" {
                        sum += 1;
                        break;
                    }
                    if line.len() - pos >= 5 && &line[pos + 1..pos + 5] == "erht" {
                        sum += 3;
                        break;
                    }
                    if line.len() - pos >= 4 && &line[pos + 1..pos + 4] == "vif" {
                        sum += 5;
                        break;
                    }
                    if line.len() - pos >= 4 && &line[pos + 1..pos + 4] == "nin" {
                        sum += 9;
                        break;
                    }
                }
                'o' => {
                    if line.len() - pos >= 3 && &line[pos + 1..pos + 3] == "wt" {
                        sum += 2;
                        break;
                    }
                }
                'r' => {
                    if line.len() - pos >= 4 && &line[pos + 1..pos + 4] == "uof" {
                        sum += 4;
                        break;
                    }
                }
                'x' => {
                    if line.len() - pos >= 3 && &line[pos + 1..pos + 3] == "is" {
                        sum += 6;
                        break;
                    }
                }
                'n' => {
                    if line.len() - pos >= 5 && &line[pos + 1..pos + 5] == "eves" {
                        sum += 7;
                        break;
                    }
                }
                't' => {
                    if line.len() - pos >= 5 && &line[pos + 1..pos + 5] == "hgie" {
                        sum += 8;
                        break;
                    }
                }
                _ => (),
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests{
    #[test]
    fn test_day_1b(){
        let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;
    assert_eq!(super::day_1b(input), 281_u32);
    }
}
