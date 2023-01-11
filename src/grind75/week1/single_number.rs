// Problem: https://leetcode.com/problems/single-number/
pub fn single_number(nums: Vec<i32>) -> i32 {
    nums.iter().fold(0, |result, num| result ^ num)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(single_number(vec![2, 2, 1, 1, 3]), 3)
    }

    #[test]
    fn example_2() {
        assert_eq!(single_number(vec![2, 2, 3, 1, 3]), 1)
    }

}
