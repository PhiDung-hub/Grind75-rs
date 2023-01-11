// Problem: https://leetcode.com/problems/add-binary/
use std::iter;

pub fn add_binary(a: String, b: String) -> String {
    let (mut a, mut b) = (a, b);
    if a.len() < b.len() {
        (a, b) = (b, a);
    }

    let mut carry = 0;
    let mut cur_sum = 0;

    let mut a_iter = a.as_bytes().iter().rev();
    let b_iter = b.as_bytes().iter().rev().chain(iter::repeat(&b'0'));
    let mut zipped_iter = a_iter.by_ref().zip(b_iter);

    let mut char_vec = zipped_iter
        .by_ref()
        .take(b.len())
        .map(|(char_a, char_b)| {
            cur_sum = (*char_a - b'0') + (*char_b - b'0') + carry;
            carry = cur_sum / 2;
            (cur_sum % 2 + b'0') as char
        })
        .collect::<Vec<char>>();

    if a.len() == b.len() {
        if carry == 1 {
            char_vec.push('1');
        }
        return char_vec.iter().rev().collect::<String>();
    }

    let mut remaining_vec: Vec<char> = Vec::new();
    while let Some((char_a, _)) = zipped_iter.next() {
        cur_sum = (*char_a - b'0') + carry;
        carry = cur_sum / 2;
        remaining_vec.push((cur_sum % 2 + b'0') as char);
        // Stop early if no carry
        if carry == 0 {
            let a_left_over = a_iter.map(|c| *c as char).collect::<Vec<char>>();
            remaining_vec.extend_from_slice(a_left_over.as_ref());
            break;
        }
    }

    // Still a cary leftover even tho len_a > len_b
    if carry == 1 {
        remaining_vec.push('1');
    }

    let char_string = char_vec.iter().rev().collect::<String>();
    let mut remaining_string = remaining_vec.iter().rev().collect::<String>();
    remaining_string.push_str(char_string.as_ref());
    remaining_string
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn basic_cases() {
        assert_eq!(add_binary("10001".to_string(), "1".to_string()), "10010".to_string());
        assert_eq!(add_binary("10001".to_string(), "11".to_string()), "10100".to_string());
    }

    #[test]
    fn carry_left_over_and_unequal_len() {
        assert_eq!(add_binary("1111".to_string(), "1".to_string()), "10000".to_string());
        assert_eq!(add_binary("100000000000000000000000000001".to_string(), "1".to_string()), "100000000000000000000000000010".to_string());
    }

    #[test]
    fn equal_len() {
        assert_eq!(add_binary("10001".to_string(), "11111".to_string()), "110000".to_string());
        assert_eq!(add_binary("11111".to_string(), "11111".to_string()), "111110".to_string());
    }
}
