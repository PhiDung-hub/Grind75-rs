// Problem: https://leetcode.com/problems/valid-anagram/
use std::collections::HashMap;
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map: HashMap<char, i32> = HashMap::new();
    for (s, t) in s.chars().zip(t.chars()) {
        if s == t {
            continue;
        }
        // let s_check = map.entry(s).key();
        // map.get_mut(s_check);
        map.entry(s).and_modify(|e| *e += 1).or_insert(1);
        map.entry(t).and_modify(|e| *e -= 1).or_insert(-1);
    }
    !map.values().any(|e| *e != 0)
}

pub fn is_anagram_functional(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map: HashMap<char, i32> = HashMap::new();

    s.chars().zip(t.chars()).for_each(|(s, t)| {
        if s == t {
            return;
        }
        map.entry(s).and_modify(|e| *e += 1).or_insert(1);
        map.entry(t).and_modify(|e| *e -= 1).or_insert(-1);
    });
    !map.values().any(|e| *e != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let left_string = String::from("anagram");
        let right_string = String::from("nagaram");
        assert!(is_anagram(left_string.clone(), right_string.clone()));
        assert!(is_anagram_functional(left_string, right_string));
    }

    #[test]
    fn example_2() {
        let left_string = String::from("rat");
        let right_string = String::from("car");
        assert!(!is_anagram(left_string.clone(), right_string.clone()));
        assert!(!is_anagram_functional(left_string, right_string));
    }

    #[test]
    fn example_3() {
        let left_string = String::from("ana");
        let right_string = String::from("nan");
        assert!(!is_anagram(left_string.clone(), right_string.clone()));
        assert!(!is_anagram_functional(left_string, right_string));
    }
}
