use std::cmp::min;

fn one_away(s1: String, s2: String) -> bool {
    let mut dp = vec![vec![0; s2.len() + 1]; s1.len() + 1];
    for i in 0..s1.len() + 1 {
        for j in 0..s2.len() + 1 {
            if i == 0 {
                dp[i][j] = j;
                continue;
            }
            if j == 0 {
                dp[i][j] = i;
                continue;
            }
            if s1[i - 1..i] == s2[j - 1..j] {
                dp[i][j] = min(dp[i - 1][j], min(dp[i][j - 1], dp[i - 1][j - 1]))
            } else {
                dp[i][j] = min(
                    dp[i - 1][j] + 1,
                    min(dp[i][j - 1] + 1, dp[i - 1][j - 1] + 1),
                )
            }
        }
    }
    dp[s1.len()][s2.len()] <= 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove() {
        assert_eq!(one_away("pale".to_string(), "ple".to_string()), true)
    }

    #[test]
    fn test_insert() {
        assert_eq!(one_away("pale".to_string(), "pales".to_string()), true)
    }

    #[test]
    fn test_replace() {
        assert_eq!(one_away("pale".to_string(), "bale".to_string()), true)
    }

    #[test]
    fn test_ng() {
        assert_eq!(one_away("pale".to_string(), "bake".to_string()), false)
    }
}
