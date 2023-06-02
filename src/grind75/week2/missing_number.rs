// Problem: https://leetcode.com/problems/missing-number/
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let n = nums.len() as i32;
    (n * (n + 1)) / 2 - nums.iter().sum::<i32>()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(missing_number(vec![0, 1, 2, 4]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(missing_number(vec![0, 1, 3, 4]), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(missing_number(vec![0, 1, 3, 4, 5, 6, 7, 8, 9]), 2);
    }
}
