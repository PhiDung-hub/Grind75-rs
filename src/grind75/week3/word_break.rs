// Problem: https://leetcode.com/problems/word-break/

use std::cmp;
use std::collections::HashSet;

pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let mut min_word_len = usize::MAX;
    let mut max_word_len = usize::MIN;
    let mut words_set: HashSet<String> = HashSet::new();
    for w in word_dict {
        max_word_len = cmp::max(max_word_len, w.len());
        min_word_len = cmp::min(min_word_len, w.len());
        words_set.insert(w.clone());
    }

    let s_len = s.len();
    let mut dp = vec![false; s_len];

    for left in 0..s_len + 1 - min_word_len {
        for guessed_word_len in min_word_len..=max_word_len {
            let right = left + guessed_word_len;
            if right > s_len {
                break;
            }

            let s2 = &s[left..right].to_string();
            if words_set.contains(s2) {
                match left {
                    0 => dp[right - 1] = true,
                    _ => dp[right - 1] |= dp[left - 1],
                }
            }
        }
    }
    dp[s_len - 1]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_word_break() {
        let words = ["leet", "code"].map(String::from).to_vec();
        let s = "leetcode".to_string();
        assert!(word_break(s, words));
    }

    #[test]
    fn test_word_break_02() {
        let words = ["apple", "pen"].map(String::from).to_vec();
        let s = "applepenapple".to_string();
        assert!(word_break(s, words));
    }

    #[test]
    fn test_word_break_03() {
        let words = ["cats", "dog", "sand", "and", "cat"].map(String::from).to_vec();
        let s = "catsandog".to_string();
        assert!(!word_break(s, words));
    }
}
