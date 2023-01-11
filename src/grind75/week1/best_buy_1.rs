// Problem: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() < 2 {
        return 0;
    }

    let mut profit: i32 = 0;
    let mut min_price_so_far: i32 = prices[0];
    for day in 1..prices.len() {
        if prices[day] < min_price_so_far {
            min_price_so_far = prices[day];
        }
        profit = std::cmp::max(profit, prices[day] - min_price_so_far);
    }
    profit
}

pub fn max_profit_functional(prices: Vec<i32>) -> i32 {
    prices
        .into_iter()
        .fold((0, i32::MAX), |(profit, min_price_seen), price_today| {
            (profit.max(price_today - min_price_seen), min_price_seen.min(price_today))
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_increase() {
        assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(max_profit_functional(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn price_decrease() {
        assert_eq!(max_profit(vec![5, 4, 3, 2, 1]), 0);
        assert_eq!(max_profit_functional(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn price_1_trough() {
        assert_eq!(max_profit(vec![1, 4, 1, 3, 7]), 6);
        assert_eq!(max_profit_functional(vec![1, 4, 1, 3, 7]), 6);
    }

    #[test]
    fn price_multiple_troughs() {
        assert_eq!(max_profit(vec![7, 1, 4, 1, 3, 7, 9, 2, 10]), 9);
        assert_eq!(max_profit_functional(vec![7, 1, 4, 1, 3, 7, 9, 2, 10]), 9);
    }
}
