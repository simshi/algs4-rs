use std::cmp::{max, min};

pub fn stock_single(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut lowest = prices[0];
    for &v in prices.iter().skip(1) {
        lowest = min(lowest, v);
        max_profit = max(max_profit, v - lowest);
    }

    max_profit
}

/// Stock buy sell unlimited
///
/// <pre>
///               2           5
/// N(不持有状态)   0 -->  继续观望0     \
///                 \/ sell 卖掉 5-2=3 -> N=max(0，3)=3
///                 /\
///                /  v buy 买入-5  \
/// H(持有状态)   -2 ->  继续持有-2   -> H=max(-5,-2)=-2
/// </pre>
pub fn stock_unlimited(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let (mut dp_none, mut dp_hold) = (0, -prices[0]);
    for &p in prices.iter().skip(1) {
        (dp_none, dp_hold) = (max(dp_none, dp_hold + p), max(dp_hold, dp_none - p));
    }

    max(dp_none, dp_hold)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single() {
        assert_eq!(0, stock_single(vec![1]));
        assert_eq!(1, stock_single(vec![1, 2]));
        assert_eq!(0, stock_single(vec![2, 1]));
        assert_eq!(5, stock_single(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(0, stock_single(vec![7, 6, 4, 3, 1]));
    }

    #[test]
    fn test_unlimit() {
        assert_eq!(0, stock_unlimited(vec![1]));
        assert_eq!(1, stock_unlimited(vec![1, 2]));
        assert_eq!(0, stock_unlimited(vec![2, 1]));
        assert_eq!(7, stock_unlimited(vec![7, 1, 5, 3, 6, 4]));
        assert_eq!(4, stock_unlimited(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, stock_unlimited(vec![7, 6, 4, 3, 1]));
    }
}
