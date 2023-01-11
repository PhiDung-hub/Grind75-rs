// Problem: https://leetcode.com/problems/binary-search
pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let (mut low, mut high) = (0, n - 1);
    let mut look_idx: usize;

    while low <= high {
        look_idx = (low + high) / 2;
        if target > nums[look_idx] {
            low = look_idx + 1;
        } else if target < nums[look_idx] {
            if look_idx < 1 { // overflow on usize might occur after
                return -1;
            }
            high = look_idx - 1;
        } else {
            return look_idx as i32;
        }

    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 9), 4)
    }

    #[test]
    fn example_2() {
        assert_eq!(search(vec![-1, 0, 3, 5, 9, 12], 2), -1)
    }

    #[test]
    fn example_3() {
        assert_eq!(search(vec![12], 2), -1)
    }

    #[test]
    fn example_4() {
        assert_eq!(search(vec![2], 12), -1)
    }
}
