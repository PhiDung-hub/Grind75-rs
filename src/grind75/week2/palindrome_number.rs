// Problem: https://leetcode.com/problems/palindrome-number/
pub fn is_palindrome(x: i32) -> bool { // also verify for positive
    if x == 0 {
        return true;
    }

    fn reverse_number(mut x: i32) -> i32 {
        let is_positive = x >= 0;
        x = if is_positive { x } else { -x };

        let mut reverse_x = 0;
        while x > 0 {
            reverse_x = 10 * reverse_x + x % 10;
            x /= 10;
        }

        if is_positive {
            reverse_x
        } else {
            -reverse_x
        }
    }

    x - reverse_number(x) == 0
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(-121), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(is_palindrome(789), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(is_palindrome(0), true);
    }
}
