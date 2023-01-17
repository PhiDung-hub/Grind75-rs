// Problem: https://leetcode.com/problems/coin-change/
pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    if amount == 0 {
        return 0;
    }

    let amount = amount as usize;
    let mut dp = vec![i32::MAX - 1; amount + 1];
    dp[0] = 0;

    for &value in coins.iter() {
        let value = value as usize;
        for j in value..=amount {
            dp[j] = dp[j].min(1 + dp[j - value]);
        }
    }

    if dp[amount] == i32::MAX - 1 {
        -1
    } else {
        dp[amount]
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(coin_change(vec![1, 2, 5], 11), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(coin_change(vec![2, 5], 3), -1);
    }
}
