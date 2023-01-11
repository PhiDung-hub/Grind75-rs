// Problem: https://leetcode.com/problems/longest-palindrome/
use std::collections::HashMap;
pub fn longest_palindrome(s: String) -> i32 {
    let mut char_frequency: HashMap<char, usize> = HashMap::new();
    s.chars().for_each(|c| {
        *char_frequency.entry(c).or_insert(0) += 1;
    });

    let mut max_len = char_frequency.values().fold(0, |mut palindrome_len, &frequency| {
        palindrome_len += frequency;
        if frequency % 2 != 0 {
            palindrome_len -= 1;
        }
        palindrome_len
    });

    if max_len < s.len() {
        max_len += 1;
    }

    max_len as i32
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(longest_palindrome("aabbcdeff".to_string()), 7);
    }

    #[test]
    fn example_2() {
        assert_eq!(longest_palindrome("a".to_string()), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(longest_palindrome("aabbcccddeff".to_string()), 11);
    }
}
