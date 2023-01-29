// Problem: https://leetcode.com/problems/permutations/

use std::collections::HashSet;

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() == 1 {
        return vec![vec![nums[0]]];
    }
    let mut result = vec![];
    let mut explore_stack = Vec::<(Vec<i32>, HashSet<usize>)>::new();

    for (idx, &num) in nums.iter().enumerate() {
        explore_stack.push((vec![num], HashSet::from([idx])));
    }

    let n = nums.len();

    while let Some((cur_vec, included_set)) = explore_stack.pop() {
        for i in 0..n {
            if !included_set.contains(&i) {
                let mut new_vec = cur_vec.clone();
                new_vec.push(nums[i]);
                let mut new_set = included_set.clone();
                new_set.insert(i);
                if new_set.len() == n {
                    result.push(new_vec.clone());
                }
                explore_stack.push((new_vec, new_set));
            }
        }
    }

    result.reverse();

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(permute([1, 2, 3].to_vec()), [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]);
    }

    #[test]
    fn example_2() {
        assert_eq!(permute([1, 3].to_vec()), [[1, 3], [3, 1]]);
    }

    #[test]
    fn example_4() {
        assert_eq!(permute([1].to_vec()), [[1]]);
    }
    #[test]
    fn example_3() {
        assert_eq!(permute([1, 5, 3].to_vec()), [[1, 5, 3], [1, 3, 5], [5, 1, 3], [5, 3, 1], [3, 1, 5], [3, 5, 1]]);
    }
}
