// Problem: https://leetcode.com/problems/valid-parentheses
use std::collections::VecDeque;

pub fn valid_parentheses(s: String) -> bool {
    let mut q: VecDeque<char> = VecDeque::new();
    for c in s.chars() {
        match c {
            '(' => q.push_back(')'),
            '[' => q.push_back(']'),
            '{' => q.push_back('}'),
            ')' | ']' | '}' => match q.back() {
                Some(front) => {
                    if c != *front {
                        return false;
                    }
                    q.pop_back();
                }
                None => {
                    return false;
                }
            },
            _ => panic!("Invalid character"),
        }
    }
    q.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert!(!valid_parentheses(String::from("(())]")));
    }

    #[test]
    fn example_2() {
        assert!(!valid_parentheses(String::from("(")));
    }

    #[test]
    fn example_3() {
        assert!(!valid_parentheses(String::from("(({]}))")));
    }

    #[test]
    fn example_4() {
        assert!(valid_parentheses(String::from("(({}))")));
    }
}
