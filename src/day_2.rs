pub fn day_2() {
    let input = include_str!("day_2_data.txt");
    println!("day 2a {}", day_2a(input));
}

fn day_2a(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.split("\n"){
        let (game_str, data_str) =  line.split_at(line.find(":").unwrap());
        let game_num = game_str[game_str.find("Game ").unwrap() + 5..].parse::<u32>().unwrap();
        let mut possible = true;
        for grab_str in data_str[1..].split(";"){
            for count_str in grab_str.split(","){
                let count_str = &count_str[1..];
                let (num, color) = count_str.split_at(count_str.find(" ").unwrap());
                let color = color.trim();
                let num = num.parse::<u32>().unwrap();
                possible &= match color {
                    "red" => num <= 12,
                    "green" => num <= 13,
                    "blue" => num <= 14,
                    other => panic!("Got bad color with len {}: {}", other.len(), &other),
                };
            }
        }
        if possible {
            sum += game_num;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(super::day_2a(input), 8);
    }
}
