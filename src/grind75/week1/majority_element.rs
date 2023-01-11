// Problem: https://leetcode.com/problems/majority-element/
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut m = nums[0];
    let mut tracker = 0;
    for num in nums {
        if tracker == 0 {
            m = num;
            tracker = 1;
        } else if num == m {
            tracker += 1;
        } else {
            tracker -= 1;
        }
    }
    m
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(majority_element(vec![1, 2, 2, 3, 2]), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(majority_element(vec![3, 2, 3, 3, 2]), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(majority_element(vec![1]), 1);
    }
}
