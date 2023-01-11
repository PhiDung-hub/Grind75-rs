// Problem: https://leetcode.com/problems/roman-to-integer/
pub fn roman_to_int(s: String) -> i32 {
    let mut int = 0;
    let iter = s.chars().zip(s.chars().skip(1).chain(std::iter::repeat('#'))).take(s.len());
    iter.for_each(|(c, next_c)| match c {
        'M' => {
            int += 1000;
        }
        'D' => {
            int += 500;
        }
        'C' => {
            if next_c == 'D' || next_c == 'M' {
                int -= 100;
            } else {
                int += 100;
            }
        }
        'L' => {
            int += 50;
        }
        'X' => {
            if next_c == 'L' || next_c == 'C' {
                int -= 10;
            } else {
                int += 10;
            }
        }
        'V' => {
            int += 5;
        }
        'I' => {
            if next_c == 'V' || next_c == 'X' {
                int -= 1;
            } else {
                int += 1;
            }
        }
        _ => {}
    });
    int
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(roman_to_int("MMCMLII".to_string()), 2952);
    }

    #[test]
    fn example_2() {
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
    }

    #[test]
    fn example_3() {
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
