// Problem: https://leetcode.com/problems/two-sum
use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut memo: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        if memo.contains_key(&nums[i]) {
            return vec![*memo.get(&nums[i]).unwrap(), i as i32];
        } else {
            memo.insert(target - nums[i], i as i32);
        }
    }

    vec![0, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn example_3() {
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
