pub fn max_profit_functional(prices: Vec<i32>) -> i32 {
    prices
        .into_iter()
        .fold((0, 0, i32::MIN), |(a, b, c), x| (a.max(c + x), b.max(a), c.max(b - x)))
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn price_increase() {
        assert_eq!(max_profit_functional(vec![1, 2, 3, 4, 5]), 4);
    }

    #[test]
    fn price_decrease() {
        assert_eq!(max_profit_functional(vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn price_1_trough() {
        assert_eq!(max_profit_functional(vec![1, 4, 3, 1, 3, 7]), 9);
    }

    #[test]
    fn price_multiple_troughs() {
        assert_eq!(max_profit_functional(vec![7, 1, 4, 1, 3, 7, 9, 2, 10]), 15);
    }
}
