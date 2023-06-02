// Problem: https://leetcode.com/problems/valid-palindrome

pub fn is_palindrome(s: String) -> bool {
    let mut string = s;
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
        assert!(is_palindrome(test_string.clone()));
        assert!(is_palindrome_functional(test_string));
    }

    #[test]
    fn example_2() {
        let test_string = String::from("race a car");
        assert!(!is_palindrome(test_string.clone()));
        assert!(!is_palindrome_functional(test_string));
    }

    #[test]
    fn example_3() {
        let test_string = String::from(" ");
        assert!(is_palindrome(test_string.clone()));
        assert!(is_palindrome_functional(test_string));
    }
}
