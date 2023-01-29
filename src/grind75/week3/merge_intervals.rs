// Problem: https://leetcode.com/problems/merge-intervals/

pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    intervals.sort_by_key(|interval| interval[0]);

    let mut result = Vec::<Vec<i32>>::new();
    let (mut temp_start, mut temp_end) = (intervals[0][0], intervals[0][1]);

    for interval in intervals {
        let (start, end) = (interval[0], interval[1]); 
        if start > temp_end {
            // next inerval is not merged, push in result and start a new one.
            result.push(vec![temp_start, temp_end]);
            temp_start = start;
            temp_end = end;
        } else if end > temp_end {
            // merge end of next interval
            temp_end = end;
        }
    }

    result.push(vec![temp_start, temp_end]);

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let intervals = [[3, 5], [1, 4], [2, 7]].map(|item| item.to_vec()).to_vec();
        assert_eq!(merge(intervals), [[1, 7]]);
    }

    #[test]
    fn example_2() {
        let intervals = [[3, 5], [1, 4], [2, 4], [6, 9], [10, 15]].map(|item| item.to_vec()).to_vec();
        assert_eq!(merge(intervals), [[1, 5], [6, 9], [10, 15]]);
    }

    #[test]
    fn example_3() {
        let intervals = [[2, 7]].map(|item| item.to_vec()).to_vec();
        assert_eq!(merge(intervals), [[2, 7]]);
    }
}
