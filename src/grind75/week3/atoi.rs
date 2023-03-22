// Problem: https://leetcode.com/problems/string-to-integer-atoi/

pub fn atoi(s: String) -> i32 {
    let mut result = 0;
    let mut is_negative = false;
    let mut graceful = true;
    for char_ in s.chars() {
        match char_ {
            '0'..='9' => {
                graceful = false;
                let char_as_int = (char_ as u32 - '0' as u32) as i32;

                // NOTE: other solution uses u32 and saturating ops result, but it's not intended
                // for this problem since we are expected to handles overflow case manually:
                // 1. use `saturating` ops on u32
                // 2. directly compares with i32::MAX / i32::MIN.
                let need_damp_negative = (-result < i32::MIN / 10) || ((-result == i32::MIN / 10) && char_as_int >= 8); // -2147483648
                if is_negative && need_damp_negative {
                    return i32::MIN;
                }

                let need_damp_positive = result > i32::MAX / 10 || (result == i32::MAX / 10 && char_as_int >= 7); // 2147483647
                if !is_negative && need_damp_positive {
                    return i32::MAX;
                }

                result = result * 10 + (char_ as u32 - '0' as u32) as i32;
            }
            ' ' | '-' | '+' => {
                if !graceful {
                    break;
                }
                if char_ == ' ' {
                    continue;
                }
                if char_ == '-' {
                    is_negative = true;
                    graceful = false;
                }
                if char_ == '+' {
                    graceful = false;
                }
            }
            _ => break,
        }
    }

    if is_negative {
        -result
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let string = "++42".to_string();
        assert_eq!(atoi(string), 0);
    }

    #[test]
    fn example_2() {
        let string = "    +42-- is the future".to_string();
        assert_eq!(atoi(string), 42);
    }

    #[test]
    fn example_3() {
        let string = " -42 here............".to_string();
        assert_eq!(atoi(string), -42);
    }

    #[test]
    fn example_4() {
        let string = "s42 here............".to_string();
        assert_eq!(atoi(string), 0);
    }

    #[test]
    fn margin_negative() {
        let string = "-2147483645".to_string();
        assert_eq!(atoi(string), -2147483645);

        let string = "-2147483648".to_string(); // exactly min number
        assert_eq!(atoi(string), i32::MIN);

        let string = "-2147483649".to_string(); // exactly min number
        assert_eq!(atoi(string), i32::MIN);

        let string = "-91283472332".to_string();
        assert_eq!(atoi(string), i32::MIN);
    }

    #[test]
    fn margin_positive() {
        let string = "2147483645".to_string();
        assert_eq!(atoi(string), 2147483645);

        let string = "2147483647".to_string();
        assert_eq!(atoi(string), i32::MAX);

        let string = "2147483648".to_string();
        assert_eq!(atoi(string), i32::MAX);

        let string = "912834723322".to_string();
        assert_eq!(atoi(string), i32::MAX);
    }
}
