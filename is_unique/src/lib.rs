use std::collections::HashSet;
use std::iter::FromIterator;

fn is_unique(s: String) -> bool {
    let hash: HashSet<char> = HashSet::from_iter(s.chars());
    s.len() == hash.len()
}

fn is_unique_no_data_structure(s: String) -> bool {
    let mut check_duplicate = String::new();
    for c in s.chars() {
        if check_duplicate.contains(c) {
            return false;
        }
        check_duplicate.push(c);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique() {
        assert_eq!(is_unique("abjgh8743".to_string()), true);
        assert_eq!(is_unique_no_data_structure("abjgh8743".to_string()), true);
    }

    #[test]
    fn is_duplicated() {
        assert_eq!(is_unique("1231".to_string()), false);
        assert_eq!(is_unique_no_data_structure("1231".to_string()), false);
    }
}
