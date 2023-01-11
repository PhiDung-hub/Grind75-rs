// Problem: https://leetcode.com/problems/3sum/
use std::collections::HashMap;

pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    if nums.len() < 3 {
        return vec![];
    }

    nums.sort();
    if nums[0] > 0 {
        return vec![];
    }

    let mut result = Vec::with_capacity(nums.len() / 3);
    let mut index_map: HashMap<i32, i32> = HashMap::new();

    for (idx, &num) in nums.iter().enumerate() {
        index_map.insert(num, idx as i32);
    }

    let mut i: usize = 0;
    while i < nums.len() - 2 {
        if nums[i] > 0 {
            break;
        }
        let mut j: usize = i + 1;
        while j < nums.len() - 1 {
            let target = -(nums[i] + nums[j]);
            if let Some(match_idx) = index_map.get(&target) {
                if *match_idx > j as i32 {
                    result.push(vec![nums[i], nums[j], target]);
                }
            }
            j = *index_map.get(&nums[j]).unwrap() as usize + 1; // update to skip duplicates
            j += 1;
        }
        i = *index_map.get(&nums[i]).unwrap() as usize + 1; // update to skip duplicates
        i += 1;
    }

    return result;
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![[-1, -1, 2], [-1, 0, 1]]);
    }

    #[test]
    fn example_2() {

        assert_eq!(three_sum(vec![-1, 0, 1, 2, -1, -4]), vec![[-1, -1, 2], [-1, 0, 1]]);
    }

    #[test]
    fn example_3() {}
}
