// Problem: https://leetcode.com/problems/sort-colors/description/
pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut lo, mut mid, mut hi) = (0, 0, nums.len() - 1);
    while mid <= hi {
        if nums[mid] == 0 {
            nums.swap(mid, lo);
            lo += 1;
            mid += 1;
        } else if nums[mid] == 2 {
            nums.swap(mid, hi);
            if hi == 0 {
                return;
            }
            hi -= 1;
        } else {
            mid += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![0, 0, 1, 1, 2, 2]);
    }

    #[test]
    fn example_2() {
        let mut nums = vec![2, 2];
        sort_colors(&mut nums);
        assert_eq!(nums, vec![2, 2]);
    }

    #[test]
    fn example_3() {}
}
