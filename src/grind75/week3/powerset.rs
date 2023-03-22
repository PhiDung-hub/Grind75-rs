// Problem:

pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answers: Vec<Vec<i32>> = vec![vec![]];

    for &num in nums.iter() {
        let answers_clone = answers.clone();
        for set in answers_clone.iter() {
            let mut new_set = set.clone();
            new_set.push(num);
            answers.push(new_set);
        }
    }
    answers
}

// NOTE: leet code exited with SIGSEGV (segmentation fault) as of 22 March 2023.
pub fn subsets_optimized_space(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut answers: Vec<Vec<i32>> = vec![vec![]];

    for &num in nums.iter() {
        // let answers_clone = answers.clone();
        let mut ptr = answers.as_mut_ptr();
        let n = answers.len();
        let end = unsafe { ptr.add(n) };
        while ptr < end {
            let set = unsafe { &*ptr };
            let mut new_set = set.clone();
            new_set.push(num);
            answers.push(new_set);
            unsafe {
                ptr = ptr.add(1);
            }
        }
    }
    answers
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let nums = vec![1, 2, 3];
        let result = &vec![vec![], vec![1], vec![2], vec![1, 2], vec![3], vec![1, 3], vec![2, 3], vec![1, 2, 3]];
        assert_eq!(subsets(nums.clone()), *result);
        assert_eq!(subsets_optimized_space(nums), *result);
    }

    #[test]
    fn example_2() {
        let nums = vec![1, 2, 4];
        let result = &vec![vec![], vec![1], vec![2], vec![1, 2], vec![4], vec![1, 4], vec![2, 4], vec![1, 2, 4]];
        assert_eq!(subsets(nums.clone()), *result);
        assert_eq!(subsets_optimized_space(nums), *result);
    }

    #[test]
    fn example_3() {
        let nums = vec![1];
        let result = &vec![vec![], vec![1]];
        assert_eq!(subsets(nums.clone()), *result);
        assert_eq!(subsets_optimized_space(nums), *result);
    }

    // #[test]
    // fn try_unsafe_with_sigsegv_on_leetcode() {
    //     let nums = vec![1, 2, 3, 4, 5, 6, 7];
    //     println!("{:?}", subsets(nums)); // result is valid
    //     panic!();
    // }
}
