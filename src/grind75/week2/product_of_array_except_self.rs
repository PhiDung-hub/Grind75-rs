// Problem: https://leetcode.com/problems/product-of-array-except-self/
use std::mem;

pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let prefix_prod = nums.iter().scan(1, |state, &x| Some(mem::replace(state, *state * x)));

    let suffix_prod = nums.iter().rev().scan(1, |state, &x| Some(mem::replace(state, *state * x)));

    prefix_prod
        .zip(suffix_prod.collect::<Vec<i32>>().into_iter().rev())
        .map(|(prefix_elem, suffix_elem)| suffix_elem * prefix_elem)
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(product_except_self(vec![1, 2, 3, 4]), [24, 12, 8, 6]);
    }

    #[test]
    fn example_2() {
        assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), [0, 0, 9, 0, 0]);
    }
}
