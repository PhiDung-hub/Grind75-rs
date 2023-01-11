// Problem: https://leetcode.com/problems/maximum-subarray/

pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    let result = nums.iter().fold((i32::MIN, 0), |(mut sum_max_sub_array, mut accumulator), &num| {
        accumulator = num.max(num + accumulator);
        sum_max_sub_array = sum_max_sub_array.max(accumulator);

        (sum_max_sub_array, accumulator)
    });
    result.0
}

pub fn max_sub_array_with_bound(nums: Vec<i32>) -> i32 {
    let result =
        nums.iter()
            .fold((i32::MIN, 0, 0, 0, 0), |(mut sum_max_sub_array, mut accumulator, mut left_bound, mut right_bound, mut idx), num| {
                accumulator += num;
                idx += 1;
                if accumulator < *num {
                    left_bound = idx;
                    right_bound = idx;
                    accumulator = *num;
                }
                if sum_max_sub_array < accumulator {
                    right_bound = idx;
                    sum_max_sub_array = accumulator;
                }

                (sum_max_sub_array, accumulator, left_bound, right_bound, idx)
            });
    result.0
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sub_array(vec![4, -1, 2, 5, -5, 4]), 10);
    }

    #[test]
    fn example_2() {
        assert_eq!(max_sub_array(vec![-1]), -1);
    }
}
