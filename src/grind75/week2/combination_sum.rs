// Problem: https://leetcode.com/problems/combination-sum/

pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let n = candidates.len();
    candidates.sort();

    let mut exploration_queue = vec![(vec![], 0, 0)];
    while let Some((cur_vec, cur_sum, idx)) = exploration_queue.pop() {
        for (i, candidate) in candidates.iter().enumerate().take(n).skip(idx) {
            let next_sum = cur_sum + candidate;
            if next_sum > target {
                break;
            }

            let mut next_vec = cur_vec.clone();
            next_vec.push(*candidate);

            if next_sum == target {
                result.push(next_vec.clone());
            }
            exploration_queue.push((next_vec, next_sum, i));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let result = [[7].to_vec(), [2, 2, 3].to_vec()].to_vec();
        assert_eq!(combination_sum([2, 3, 6, 7].to_vec(), 7), result);
    }

    #[test]
    fn example_2() {
        assert_eq!(combination_sum([2].to_vec(), 1), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn example_3() {
        let result = [[3, 5].to_vec(), [2, 3, 3].to_vec(), [2, 2, 2, 2].to_vec()].to_vec();
        assert_eq!(combination_sum([2, 3, 5].to_vec(), 8), result);
    }
}
