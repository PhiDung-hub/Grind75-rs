// Problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
use std::collections::*;

pub fn longest_substr(s: String) -> i32 {
    if s.len() < 2 {
        return s.len() as i32;
    }
    let mut char_map: HashMap<char, i32> = HashMap::new();
    let (mut substr_len, mut left_idx, mut result) = (0, 0, i32::MIN);
    let vec_s = s.chars().collect::<Vec<char>>();

    for right_idx in 0..s.len() {
        char_map.entry(vec_s[right_idx]).and_modify(|e| *e += 1).or_insert(1);
        substr_len += 1;

        while *char_map.get(&vec_s[right_idx]).unwrap() > 1 {
            char_map.entry(vec_s[left_idx]).and_modify(|e| *e -= 1);
            left_idx += 1;
            substr_len -= 1;
        }

        result = result.max(substr_len);
    }

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let input_str = "abcaddbc".to_string();
        assert_eq!(longest_substr(input_str), 4);
    }

    #[test]
    fn example_2() {
        let input_str = "abcbc".to_string();
        assert_eq!(longest_substr(input_str), 3);
    }

    #[test]
    fn example_3() {
        let input_str = "aaaa".to_string();
        assert_eq!(longest_substr(input_str), 1);
    }
}
