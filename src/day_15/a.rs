fn hash_str(s: &str) -> usize {
    let mut val = 0;
    s.chars().for_each(|c| {
        val += c as usize;
        val *= 17;
        val %= 256;
    });
    val
}

pub fn run(input: &str) -> usize {
    input.split(",").map(|s| hash_str(s)).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run(input), 1320);
    }
}
