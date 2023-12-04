pub fn day_4() {
    let input = include_str!("day_4_data.txt");
    println!("day 4a {}", day_4a(input));
    // println!("day 4b {}", day_4b(input));
}

fn process_4a_line(line: & str) -> u32{
    let (winners, nums) = line.split_once(" | ").unwrap();
    let mut set = std::collections::HashSet::new();
    let (_, winners) = winners.split_once(": ").unwrap();
    for winner in winners.split_whitespace() {
        let winner = winner.trim();
        set.insert(winner);
    }
    let mut score = 0_u32;
    for num in nums.split_whitespace() {
        let num = num.trim();
        if set.contains(num) {
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }
    }
    score
}

fn day_4a(input: &str) -> u32 {
    input.split("\n").map(|line| process_4a_line(line)).sum()
}

fn process_4b_line(line: & str) -> u32 {
    let (winners, nums) = line.split_once(" | ").unwrap();
    let mut set = std::collections::HashSet::new();
    let (_, winners) = winners.split_once(": ").unwrap();
    for winner in winners.split_whitespace() {
        let winner = winner.trim();
        set.insert(winner);
    }
    let mut matches = 0_u32;
    for num in nums.split_whitespace() {
        let num = num.trim();
        if set.contains(num) {
            matches += 1;
        }
    }
    matches
}

fn day_4b(input: &str) -> u32 {
    let mut card_count = counter::Counter::new();
    let mut highest = 0_usize;
    let mut last_card = 0_usize;
    for (iline, line) in input.lines().enumerate() {
        card_count[&iline] += 1;
        let matches = process_4a_line(line);
        let multiplier = card_count[&iline];
        for i in 1..matches + 1 {
            card_count[&(iline + i as usize)] += multiplier;
        }
        highest = highest.max(iline + matches);
        last_card = iline;
    }

    dbg!(&card_count);
    card_count.total()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
        assert_eq!(super::day_4a(input), 13);
        assert_eq!(super::day_4b(input), 30);
    }
}
