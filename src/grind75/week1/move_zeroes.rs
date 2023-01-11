// Problem: https://leetcode.com/problems/move-zeroes/solutions/
pub fn move_zeroes(nums: &mut Vec<i32>) {
    let mut z = Vec::new();
    nums.retain(|x| {
        if *x != 0 {
            true
        } else {
            z.push(0);
            false
        }
    });
    nums.append(&mut z);
}

pub fn move_zeroes_imperative(nums: &mut Vec<i32>) {
    let mut next_none_zero_idx = 0;
    for i in 0..nums.len() {
        if nums[i] != 0 {
            nums.swap(next_none_zero_idx, i);
            next_none_zero_idx += 1;
        }
    }
}

pub fn move_zeroes_unstable(nums: &mut Vec<i32>) {
    let mut zeroes = nums.drain_filter(|x| *x == 0).collect::<Vec<_>>();
    nums.append(&mut zeroes);
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let vec = &mut vec![1, 0, 0, 3, 4, 0, 6];
        move_zeroes(vec);
        assert_eq!(vec, &mut vec![1, 3, 4, 6, 0, 0, 0]);
        let vec = &mut vec![1, 0, 0, 3, 4, 0, 6];
        move_zeroes_unstable(vec);
        assert_eq!(vec, &mut vec![1, 3, 4, 6, 0, 0, 0]);
        let vec = &mut vec![1, 0, 0, 3, 4, 0, 6];
        move_zeroes_imperative(vec);
        assert_eq!(vec, &mut vec![1, 3, 4, 6, 0, 0, 0]);
    }

    #[test]
    fn example_2() {
        let vec = &mut vec![1, 0, 0, 0];
        move_zeroes(vec);
        assert_eq!(vec, &mut vec![1, 0, 0, 0]);
        let vec = &mut vec![1, 0, 0, 0];
        move_zeroes_unstable(vec);
        assert_eq!(vec, &mut vec![1, 0, 0, 0]);
        let vec = &mut vec![1, 0, 0, 0];
        move_zeroes_imperative(vec);
        assert_eq!(vec, &mut vec![1, 0, 0, 0]);
    }

    #[test]
    fn example_3() {
        let vec = &mut vec![1, 0, 0, 0];
        move_zeroes(vec);
        assert_eq!(vec, &mut vec![1, 0, 0, 0]);
        let vec = &mut vec![1, 0, 0, 0];
        move_zeroes_unstable(vec);
        assert_eq!(vec, &mut vec![1, 0, 0, 0]);
        let vec = &mut vec![1, 0, 0, 0];
        move_zeroes_imperative(vec);
        assert_eq!(vec, &mut vec![1, 0, 0, 0]);
    }
}
