// Problem: https://leetcode.com/problems/spiral-matrix/

enum Direction {
    Right,
    Down,
    Left,
    Up,
}

pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut result = vec![0; m * n];
    let mut cur_direction = Direction::Right;

    let (mut left, mut right, mut low, mut high) = (0, n - 1, 0, m - 1);

    let mut step_number = 0;

    while step_number < m * n {
        match cur_direction {
            Direction::Right => {
                for i in left..=right {
                    result[step_number] = matrix[low][i];
                    step_number += 1;
                }
                low += 1;
                cur_direction = Direction::Down;
            }
            Direction::Down => {
                for i in low..=high {
                    result[step_number] = matrix[i][right];
                    step_number += 1;
                }
                right -= 1;
                cur_direction = Direction::Left;
            }
            Direction::Left => {
                for i in (left..=right).rev() {
                    result[step_number] = matrix[high][i];
                    step_number += 1;
                }
                high -= 1;
                cur_direction = Direction::Up;
            }
            Direction::Up => {
                for i in (low..=high).rev() {
                    result[step_number] = matrix[i][left];
                    step_number += 1;
                }
                left += 1;
                cur_direction = Direction::Right;
            }
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
        let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]].map(|array| array.to_vec()).to_vec();
        let result = [1, 2, 3, 6, 9, 8, 7, 4, 5].to_vec();
        assert_eq!(result, spiral_order(matrix));
    }

    #[test]
    fn example_2() {
        let matrix = [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]].map(|arr| arr.to_vec()).to_vec();
        let result = [1, 2, 3, 4, 8, 12, 11, 10, 9, 5, 6, 7].to_vec();
        assert_eq!(result, spiral_order(matrix));
    }
}
