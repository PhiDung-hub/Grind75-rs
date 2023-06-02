// Problem: https://leetcode.com/problems/find-all-anagrams-in-a-string/
use std::collections::HashMap;

pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
    let mut result = Vec::<i32>::new();

    if p.len() > s.len() {
        return result;
    }

    let mut pattern = HashMap::<u8, i32>::new();
    p.as_bytes().iter().for_each(|c| {
        pattern.entry(*c).and_modify(|v| *v += 1).or_insert(1);
    });

    let mut char_matched = 0;
    let s_bytes = s.as_bytes();

    // First pass.
    for i in 0..p.len() {
        pattern.entry(s_bytes[i]).and_modify(|v| {
            *v -= 1;
            if *v >= 0 {
                char_matched += 1;
            }
        });
    }

    if char_matched == p.len() {
        result.push(0);
    }

    // update the rest.
    for i in p.len()..s.len() {
        // remove the first character of previous sliding window
        pattern.entry(s_bytes[i - p.len()]).and_modify(|v| {
            *v += 1;
            if *v > 0 {
                char_matched -= 1;
            }
        });

        // update new character of this sliding window (the last one)
        pattern.entry(s_bytes[i]).and_modify(|v| {
            *v -= 1;
            if *v >= 0 {
                char_matched += 1;
            }
        });

        if char_matched == p.len() {
            result.push((i + 1 - p.len()) as i32);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let s = "cbaebabacd".to_string();
        let p = "abc".to_string();
        assert_eq!(find_anagrams(s, p), [0, 6]);
    }

    #[test]
    fn example_2() {
        let s = "abab".to_string();
        let p = "ab".to_string();
        assert_eq!(find_anagrams(s, p), [0, 1, 2]);
    }
}
