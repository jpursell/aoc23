pub fn run_b(input: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        let input = include_str!("example_data.txt");
        assert_eq!(super::run_b(input), 0);
    }
}
