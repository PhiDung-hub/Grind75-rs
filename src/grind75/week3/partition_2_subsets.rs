// Problem: https://leetcode.com/problems/partition-equal-subset-sum/

pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: usize = nums.iter().sum::<i32>() as usize;
    if sum % 2 != 0 {
        return false;
    }
    let set_sum = sum / 2;
    if *nums.iter().max().unwrap() > set_sum as i32 {
        return false;
    }

    let mut dp = vec![false; set_sum + 1];
    dp[0] = true;

    for num in nums {
        let num = num as usize;
        for guessed_sum in (num..=set_sum).rev() {
            dp[guessed_sum] |= dp[guessed_sum - num];
        }
    }

    dp[set_sum]
}

pub fn can_partition_backtrack(mut nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % 2 != 0 {
        return false;
    }
    let set_sum = sum / 2;
    nums.sort();
    if *nums.last().unwrap() > set_sum {
        return false;
    }

    let mut visited = vec![false; nums.len()];

    fn back_track(set_sum: &i32, nums: &[i32], visited: &mut Vec<bool>, cur_sum: i32, next_idx: usize) -> bool {
        if cur_sum > *set_sum {
            return false;
        }
        if cur_sum == *set_sum {
            return true;
        }

        for i in next_idx..nums.len() {
            if visited[i] {
                continue;
            }
            // backtrack
            visited[i] = true;
            if back_track(set_sum, nums, visited, cur_sum + nums[i], i + 1) {
                return true;
            }
            visited[i] = false;
        }

        false
    }
    back_track(&set_sum, &nums, &mut visited, 0, 0)
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 5, 11, 5];
        assert!(can_partition_backtrack(nums.clone()));
        assert!(can_partition(nums));
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2, 3, 5];
        assert!(!can_partition_backtrack(nums.clone()));
        assert!(!can_partition(nums));
    }

    #[test]
    fn example_3() {
        let nums = vec![2, 2, 3, 5];
        assert!(!can_partition_backtrack(nums.clone()));
        assert!(!can_partition(nums));
    }
}
