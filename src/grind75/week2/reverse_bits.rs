// Problem: https://leetcode.com/problems/reverse-bits/
pub fn reverse_bits(mut x: u32) -> u32 {
    (0..32).fold(0u32, |mut result, _| {
        result = (result << 1) | (x & 1);
        x >>= 1;
        result
    })
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {}

    #[test]
    fn example_2() {}

    #[test]
    fn example_3() {}
}
