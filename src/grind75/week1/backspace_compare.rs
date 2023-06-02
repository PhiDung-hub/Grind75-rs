// Problem: https://leetcode.com/problems/backspace-string-compare/
pub fn backspace_compare(s: String, t: String) -> bool {
    fn get_str(s: String) -> Vec<char> {
        let mut stack = Vec::new();
        s.chars().for_each(|c| match c {
            '#' => {
                stack.pop();
            }
            _ => stack.push(c),
        });
        stack
    }
    get_str(s) == get_str(t)
}

pub fn backspace_compare_optimized(s: String, t: String) -> bool {
    let next_char_pos = |vec_chars: &[u8], mut idx: i32| -> i32 {
        let mut skip = 0;
        // only continue if next char is skipable.
        while idx >= 0 && (skip > 0 || vec_chars[idx as usize] == b'#') {
            skip += if vec_chars[idx as usize] == b'#' { 1 } else { -1 };
            idx -= 1; // continue to look backward
        }
        idx
    };

    let (mut s_idx, mut t_idx) = (s.len() as i32, t.len() as i32);
    let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());
    loop {
        // Look backward to process the string
        s_idx = next_char_pos(s_bytes, s_idx - 1);
        t_idx = next_char_pos(t_bytes, t_idx - 1);
        if s_idx < 0 || t_idx < 0 || s_bytes[s_idx as usize] != t_bytes[t_idx as usize] {
            break;
        }
    }
    s_idx == -1 && t_idx == -1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert!(backspace_compare("aa#bc".to_string(), "awe##bc".to_string()));
        assert!(backspace_compare_optimized("aa#bc".to_string(), "awe##bc".to_string()));
    }

    #[test]
    fn example_2() {
        assert!(!backspace_compare("aaaaa#bc".to_string(), "awaae##bc".to_string()));
        assert!(!backspace_compare_optimized("aaaaa#bc".to_string(), "awaae##bc".to_string()));
    }

    #[test]
    fn example_3() {
        assert!(backspace_compare("###a#bc".to_string(), "awewer######bc".to_string()));
        assert!(backspace_compare_optimized("###a#bc".to_string(), "awewer######bc".to_string()));
    }
}
