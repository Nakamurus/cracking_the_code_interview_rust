fn check_permutation(x: String, y: String) -> bool {
    let mut x: Vec<char> = x.chars().collect();
    x.sort();
    let mut y: Vec<char> = y.chars().collect();
    y.sort();
    x == y
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn is_permutation() {
        assert_eq!(
            true,
            check_permutation("abc".to_string(), "bca".to_string())
        );
    }

    #[test]
    fn not_permutation() {
        assert_eq!(
            false,
            check_permutation("abv".to_string(), "bca".to_string())
        );
    }

    #[test]
    fn different_length() {
        assert_eq!(
            false,
            check_permutation("abc1".to_string(), "bca".to_string())
        );
    }
}
