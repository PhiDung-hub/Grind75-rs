// Problem: https://leetcode.com/problems/number-of-islands/

pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
    if grid.is_empty() {
        return 0;
    }

    fn populate_island(grid: &mut Vec<Vec<char>>, i: usize, j: usize) {
        let mut explore_stack = vec![(i, j)];
        if grid[i][j] != '1' {
            return;
        }

        while let Some((i, j)) = explore_stack.pop() {
            grid[i][j] = '#'; // visited

            if i > 0 && grid[i - 1][j] == '1' {
                explore_stack.push((i - 1, j));
            }
            if j > 0 && grid[i][j - 1] == '1' {
                explore_stack.push((i, j - 1));
            }
            if i + 1 < grid.len() && grid[i + 1][j] == '1' {
                explore_stack.push((i + 1, j));
            }
            if j + 1 < grid[0].len() && grid[i][j + 1] == '1' {
                explore_stack.push((i, j + 1));
            }
        }
    }

    let mut count_islands = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '1' {
                populate_island(&mut grid, i, j);
                count_islands += 1;
            }
        }
    }

    count_islands
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        let island_grid = [
            ['1', '1', '1', '1', '0'],
            ['1', '1', '0', '1', '0'],
            ['1', '1', '0', '0', '0'],
            ['0', '1', '1', '1', '1'],
        ]
        .map(|arr| arr.to_vec())
        .to_vec();
        assert_eq!(num_islands(island_grid), 1);
    }

    #[test]
    fn example_2() {
        let island_grid = [
            ['1', '1', '0', '0', '0'],
            ['1', '0', '0', '0', '0'],
            ['0', '0', '1', '0', '0'],
            ['0', '0', '0', '1', '1'],
        ]
        .map(|arr| arr.to_vec())
        .to_vec();

        assert_eq!(num_islands(island_grid), 3);
    }

    #[test]
    fn example_3() {}
}
