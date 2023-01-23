// Problem: https://leetcode.com/problems/rotting-oranges/
use std::collections::*;

pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
    if grid.is_empty() || grid[0].is_empty() {
        return -1;
    }

    let mut time_elapsed = 0;
    let mut rotten_oranges_queue = VecDeque::<(usize, usize)>::new();
    let mut count_fresh_oranges = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 2 {
                rotten_oranges_queue.push_back((i, j));
            } else if grid[i][j] == 1 {
                count_fresh_oranges += 1;
            }
        }
    }

    let (rows, cols) = (grid.len(), grid[0].len());
    while !rotten_oranges_queue.is_empty() && count_fresh_oranges > 0 {
        time_elapsed += 1;
        for _ in 0..rotten_oranges_queue.len() {
            let (i, j) = rotten_oranges_queue.pop_front().unwrap();
            let (i, j) = (i as i32, j as i32);

            let mut explore_cell = |new_i: i32, new_j: i32| {
                if new_i < 0 || new_i >= rows as i32 || new_j < 0 || new_j >= cols as i32 {
                    return;
                }
                let (new_i, new_j) = (new_i as usize, new_j as usize);

                if grid[new_i][new_j] == 1 {
                    grid[new_i][new_j] = 2;
                    count_fresh_oranges -= 1;
                    rotten_oranges_queue.push_back((new_i, new_j));
                }
            };
            explore_cell(i, j + 1);
            explore_cell(i + 1, j);
            explore_cell(i, j - 1);
            explore_cell(i - 1, j);
        }
    }

    if count_fresh_oranges == 0 {
        time_elapsed
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let grid = [[2, 1, 1], [1, 1, 0], [0, 1, 1]].map(|arr| arr.to_vec()).to_vec();
        assert_eq!(oranges_rotting(grid), 4);
    }

    #[test]
    fn example_2() {
        let grid = [[2, 1, 1], [1, 1, 0], [1, 0, 1]].map(|arr| arr.to_vec()).to_vec();
        assert_eq!(oranges_rotting(grid), -1);
    }

    #[test]
    fn example_3() {
        let grid = [[2, 0]].map(|arr| arr.to_vec()).to_vec();
        assert_eq!(oranges_rotting(grid), 0);
    }
}
