// Problem: https://leetcode.com/problems/valid-palindrome
// use lazy_static::lazy_static;
// use regex::Regex;

pub fn is_palindrome(s: String) -> bool {
    // Regex is OVERKILL.
    // lazy_static! {
    //     static ref RE_ALPHANUMERIC: Regex = Regex::new("[a-zA-Z0-9]").unwrap();
    // }
    let mut string = s.clone();
    string.retain(|c| !c.is_whitespace() & c.is_alphanumeric());
    string = string.to_ascii_lowercase();

    let char_iter = string.chars();

    char_iter.clone().eq(char_iter.rev())
}

pub fn is_palindrome_functional(s: String) -> bool {
    let s_it = s.chars().filter_map(|c| match c.is_alphanumeric() {
        true => Some(c.to_ascii_lowercase()),
        false => None,
    });
    s_it.clone().eq(s_it.rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let test_string = String::from("A man, a plan, a canal: Panama");
        assert_eq!(is_palindrome(test_string.clone()), true);
        assert_eq!(is_palindrome_functional(test_string.clone()), true);
    }

    #[test]
    fn example_2() {
        let test_string = String::from("race a car");
        assert_eq!(is_palindrome(test_string.clone()), false);
        assert_eq!(is_palindrome_functional(test_string.clone()), false);
    }

    #[test]
    fn example_3() {
        let test_string = String::from(" ");
        assert_eq!(is_palindrome(test_string.clone()), true);
        assert_eq!(is_palindrome_functional(test_string.clone()), true);
    }
}
