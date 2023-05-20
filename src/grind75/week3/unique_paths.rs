// Problem: https://leetcode.com/problems/unique-paths/

pub fn unique_paths(m: i32, n: i32) -> i32 {
    let (m, n) = (m as usize, n as usize);
    let mut dp = vec![vec![0; n]; m];

    // Fill the first row with 1
    dp[0].fill(1);

    // Fill the first column with 1
    dp.iter_mut().for_each(|row| row[0] = 1);

    for i in 1..m {
        for j in 1..n {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    dp[m - 1][n - 1]
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(unique_paths(3, 7), 28);
    }

    #[test]
    fn example_2() {
        assert_eq!(unique_paths(3, 2), 3);
    }

    #[test]
    fn example_3() {
        assert_eq!(unique_paths(3, 3), 6);
    }
}
