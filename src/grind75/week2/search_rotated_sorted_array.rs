// Problem: https://leetcode.com/problems/search-in-rotated-sorted-array/description/

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let (mut lo, mut hi) = (0, n - 1);

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        if nums[mid] == target {
            return mid as i32;
        }

        let is_sorted_left = nums[lo] <= nums[mid]; // else nums[mid] <= nums[hi]
        let is_target_in_left = target <= nums[mid] && target >= nums[lo];
        let is_target_in_right = target >= nums[mid] && target <= nums[hi];

        if is_sorted_left {
            if is_target_in_left {
                hi = mid - 1;
            } else {
                lo = mid + 1;
            }
        } else {
            // right must be sorted, i.e., nums[mid] <= nums[hi]
            if is_target_in_right {
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(search(vec![5, 1, 3], 5), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(search(vec![5, 1, 3], 2), -1);
    }

    #[test]
    fn example_3() {
        assert_eq!(search(vec![5, 6, 7, 1, 2, 3, 4], 2), 4);
    }
}
