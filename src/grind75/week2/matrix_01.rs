// Problem: https://leetcode.com/problems/01-matrix/
use std::collections::VecDeque;

pub fn update_matrix(mut matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let (r, c) = (matrix.len(), matrix[0].len());
    let mut visited_cells: VecDeque<(usize, usize)> = VecDeque::new();
    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            if *val == 0 {
                visited_cells.push_back((i, j));
            } else {
                *val = i32::MAX; // unvisited
            }
        }
    }
    while let Some((i, j)) = visited_cells.pop_front() {
        // visit next 4 neighbors
        for d in [0, 1, 0, !0, 0].windows(2) {
            let di = i.wrapping_add(d[0]);
            let dj = j.wrapping_add(d[1]);

            if di < r && dj < c && matrix[di][dj] > matrix[i][j] {
                matrix[di][dj] = matrix[i][j] + 1;
                visited_cells.push_back((di, dj));
            }
        }
    }
    matrix
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let update_matrix = update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]);
        assert_eq!(update_matrix, vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]]);
    }

    #[test]
    fn example_2() {
        let update_matrix = update_matrix(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]);
        assert_eq!(update_matrix, vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 2, 1]]);
    }

    #[test]
    fn example_3() {
        let update_matrix = update_matrix(vec![vec![0], vec![1]]);
        assert_eq!(update_matrix, vec![vec![0], vec![1]]);
    }
}
