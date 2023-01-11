// Problem: https://leetcode.com/problems/squares-of-a-sorted-array/
pub fn sorted_squares(mut nums: Vec<i32>) -> Vec<i32> {
    let split_idx = nums.iter().fold(0, |mut split_idx, &num| {
        split_idx += if num < 0 { 1 } else { 0 };
        split_idx
    });

    let (negative_nums, positive_nums) = nums.split_at_mut(split_idx);
    negative_nums.reverse();

    fn merge_and_square_nums(n_nums: &[i32], p_nums: &[i32]) -> Vec<i32> {
        let mut result = vec![];
        let (mut i, mut j) = (0, 0);
        while i < n_nums.len() && j < p_nums.len() {
            if n_nums[i].abs() < p_nums[j] {
                result.push(n_nums[i] * n_nums[i]);
                i += 1;
            } else {
                result.push(p_nums[j] * p_nums[j]);
                j += 1;
            }
        }

        while i < n_nums.len() {
            result.push(n_nums[i] * n_nums[i]);
            i += 1;
        }
        while j < p_nums.len() {
            result.push(p_nums[j] * p_nums[j]);
            j += 1;
        }
        result
    }

    merge_and_square_nums(negative_nums, positive_nums)
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
