pub fn add(n: u64, m: u64) -> u64 {
    n + m
}

mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
