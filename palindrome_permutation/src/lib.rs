use std::collections::HashMap;

fn palindrome_permutation(s: String) -> bool {
    let mut map = HashMap::new();
    for c in s.to_lowercase().replace(" ", "").chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map.values().filter(|&x| x % 2 == 1).count() <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome_odd_length() {
        assert_eq!(true, palindrome_permutation("Tact Coa".to_string()));
    }

    #[test]
    fn is_palindrome_even_length() {
        assert_eq!(true, palindrome_permutation("AA BB".to_string()));
    }

    #[test]
    fn not_palindrome_even_length() {
        assert_eq!(false, palindrome_permutation("AC DB".to_string()));
    }

    #[test]
    fn not_palindrome_odd_length() {
        assert_eq!(false, palindrome_permutation("accdbaa".to_string()));
    }
}
