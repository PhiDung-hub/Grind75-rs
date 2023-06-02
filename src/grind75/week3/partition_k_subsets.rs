// Problem: https://leetcode.com/problems/partition-to-k-equal-sum-subsets/

pub fn can_partition(mut nums: Vec<i32>, k: i32) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % k != 0 {
        return false;
    }

    let set_sum = sum / k;
    nums.sort();
    if *nums.last().unwrap() > set_sum {
        return false;
    }

    let mut visited = vec![false; nums.len()];

    fn back_track(k: usize, set_sum: &i32, nums: &[i32], visited: &mut Vec<bool>, cur_sum: i32, next_idx: usize) -> bool {
        if k == 1 {
            return true;
        }
        if cur_sum > *set_sum {
            return false;
        }
        if cur_sum == *set_sum {
            return back_track(k - 1, set_sum, nums, visited, 0, 0);
        }

        for i in next_idx..nums.len() {
            if visited[i] {
                continue;
            }
            // backtrack
            visited[i] = true;
            if back_track(k, set_sum, nums, visited, cur_sum + nums[i], i + 1) {
                return true;
            }
            visited[i] = false;
        }

        false
    }

    back_track(k as usize, &set_sum, &nums, &mut visited, 0, 0)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![4, 3, 2, 3, 5, 2, 1];
        assert!(can_partition(nums, 4));
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2, 3, 4];
        assert!(!can_partition(nums, 3));
    }

    #[test]
    fn example_3() {
        let nums = vec![553, 450, 2412, 1735, 521, 170, 943, 87, 3200, 473, 75, 3819, 492, 324, 689, 629];
        assert!(can_partition(nums, 4));
    }
}
