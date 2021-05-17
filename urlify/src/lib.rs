fn urlify(s: String, ln: usize) -> String {
    let mut urlified = String::new();
    let splitted: Vec<&str> = s.split(" ").filter(|&x| x != "").collect();
    for i in 0..splitted.len() {
        urlified += splitted[i];
        if i != splitted.len() - 1 {
            urlified += "%20";
        }
    }
    urlified
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_ok() {
        assert_eq!(
            "Mr%20John%20Smith",
            urlify("Mr John  Smith     ".to_string(), 13)
        );
    }
}
