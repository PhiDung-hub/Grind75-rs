// Problem: https://leetcode.com/problems/contains-duplicate/
use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut set: HashSet<i32> = HashSet::new();
    nums.iter().any(|&num| !set.insert(num))
}

pub fn contains_duplicate_sort(nums: Vec<i32>) -> bool {
    let mut nums = nums;
    nums.sort_unstable();
    nums.iter().zip(nums.iter().skip(1)).any(|(&a, &b)| a == b)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn false_cases() {
        let vec1 = vec![1, 2, 3, 4, 5];
        assert_eq!(contains_duplicate(vec1.clone()), false);
        assert_eq!(contains_duplicate_sort(vec1), false);
        let vec2 = vec![1, 2, 3, 4, 5, 6,7,8,9,10];
        assert_eq!(contains_duplicate(vec2.clone()), false);
        assert_eq!(contains_duplicate_sort(vec2), false);
    }

    #[test]
    fn true_cases() {
        let vec1 = vec![3, 2, 2, 3, 3, 3, 3, 3];
        assert_eq!(contains_duplicate(vec1.clone()), true);
        assert_eq!(contains_duplicate_sort(vec1), true);

        let vec2 = vec![1, 2, 2, 4, 5];
        assert_eq!(contains_duplicate(vec2.clone()), true);
        assert_eq!(contains_duplicate_sort(vec2), true);
    }
}
