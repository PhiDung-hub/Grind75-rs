// Problem: https://leetcode.com/problems/insert-interval/
pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    let mut after = vec![];
    let mut start = new_interval[0];
    let mut end = new_interval[1];

    for curr in intervals {
        if curr[1] < new_interval[0] {
            result.push(curr);
        } else if curr[0] > new_interval[1] {
            after.push(curr);
        } else {
            start = curr[0].min(start);
            end = curr[1].max(end);
        }
    }
    result.push(vec![start, end]);
    result.append(&mut after);
    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let intervals = [[1, 3], [6, 9]].map(|v| v.to_vec()).to_vec();
        let new_interval = [2, 5].to_vec();
        assert_eq!(insert(intervals, new_interval), [[1, 5], [6, 9]]);
    }

    #[test]
    fn example_2() {
        let intervals = [[1, 2], [3, 5], [6, 7], [8, 10], [12, 16]].map(|v| v.to_vec()).to_vec();
        let new_interval = [4, 8].to_vec();
        assert_eq!(insert(intervals, new_interval), [[1, 2], [3, 10], [12, 16]]);
    }
}
