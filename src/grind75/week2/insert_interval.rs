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
    fn example_1() {}

    #[test]
    fn example_2() {}

    #[test]
    fn example_3() {}
}
