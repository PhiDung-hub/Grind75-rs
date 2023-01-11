// Problem: https://leetcode.com/problems/counting-bits/
pub fn count_bits(n: i32) -> Vec<i32> {
    (0..=n).map(|num| num.count_ones() as i32).collect()
}

pub fn count_bits_builtin(n: i32) -> Vec<i32> {
    let mut result = vec![0; 1 + n as usize];
    // dynamic programming by right shift 1 bit and plus the last bits
    (1..=n as usize).for_each(|i| result[i] = result[i >> 1] + (i & 1) as i32);
    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(count_bits(2), vec![0, 1, 1]);
        assert_eq!(count_bits_builtin(2), vec![0, 1, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(count_bits(5), vec![0, 1, 1, 2, 1, 2]);
        assert_eq!(count_bits_builtin(5), vec![0, 1, 1, 2, 1, 2]);
    }

    #[test]
    fn example_3() {
        assert_eq!(count_bits(10), vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2]);
        assert_eq!(count_bits_builtin(10), vec![0, 1, 1, 2, 1, 2, 2, 3, 1, 2, 2]);
    }
}
