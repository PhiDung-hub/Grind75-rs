// Problem: https://leetcode.com/problems/k-closest-points-to-origin/

pub fn k_closest(mut points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    points.sort_by_key(|point| point[0].pow(2) + point[1].pow(2));
    points[..k as usize].to_vec()
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let points = vec![vec![1, 1], vec![1, -2]];
        assert_eq!(k_closest(points, 1), vec![[1, 1]])
    }

    #[test]
    fn example_2() {
        let points = vec![vec![1, 1], vec![1, -2], vec![2,2], vec![1,-1]];
        assert_eq!(k_closest(points, 2), vec![[1, 1], [1, -1]])
    }
}
