fn string_compression(s: String) -> String {
    if s.len() <= 1 {
        return s;
    }

    let mut compressed = String::new();
    let s_vec: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i != s.len() {
        let c = s_vec[i];
        let mut cnt = 0;
        while i < s.len() && c == s_vec[i] {
            cnt += 1;
            i += 1;
        }
        compressed.push(c);
        compressed.push(std::char::from_digit(cnt, 10).unwrap());
    }
    if compressed.len() >= s.len() {
        return s;
    } else {
        return compressed;
    }
}

#[cfg(test)]
mod test_string_compression {
    use super::*;

    #[test]
    fn is_ok() {
        assert_eq!(
            string_compression("aabcccccaaa".to_string()),
            "a2b1c5a3".to_string()
        );
        assert_eq!(
            string_compression("aabcccccccaaaassa".to_string()),
            "a2b1c7a4s2a1".to_string()
        );
    }

    #[test]
    fn is_empty() {
        assert_eq!(string_compression("".to_string()), "".to_string());
    }

    #[test]
    fn is_ng() {
        assert_eq!(
            string_compression("abcdefg".to_string()),
            "abcdefg".to_string()
        );
    }
}
