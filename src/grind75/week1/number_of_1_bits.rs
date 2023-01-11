// Problem: https://leetcode.com/problems/number-of-1-bits/
pub fn hamming_weight_builtin(n: u32) -> i32 {
    n.count_ones() as i32
}

pub fn hamming_weight(n: u32) -> i32 {
    let mut n = n;
    let mut count = 0;
    while n > 0 {
        count += n & 1;
        n >>= 1;
    }
    count as i32
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(hamming_weight(5), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(hamming_weight(7), 3);
    }

    #[test]
    fn example_3() {}
}
