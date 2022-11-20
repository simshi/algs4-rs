//! You are given an array prices where prices[i] is the price of a given stock on the ith day.
//! Find the maximum profit
//!
//! e.g.
//! <pre>
//! Input: prices = [7, 2, 5, 1, 3]
//! Output: 5
//!   Buy 2 and sell at 5, profit = 3, buy 1 and sell at 3, profit = 2, total profit 5
//! </pre>
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
    let (mut dp_none, mut dp_hold) = (0, -prices[0]);
    for &p in prices.iter().skip(1) {
        (dp_none, dp_hold) = (max(dp_none, dp_hold + p), max(dp_hold, dp_none - p));
    }

    max(dp_none, dp_hold)
}

/// Stock buy and sell with limitation
pub fn stock_limit_2(prices: Vec<i32>) -> i32 {
    let mut dp_cash = vec![0i32; 3];
    let mut dp_hold = vec![0i32; 2];
    dp_hold[1] = -prices[0];
    dp_hold[0] = std::i32::MIN;

    // 买入时（而非卖出时）减次数
    for &p in prices.iter().skip(1) {
        // max(继续观望不买, 卖出)
        let cash_2 = dp_cash[2];
        let cash_1 = max(dp_cash[1], dp_hold[1] + p);
        let cash_0 = max(dp_cash[0], dp_hold[0] + p);
        // max(继续持有不卖, 买入)
        let hold_1 = max(dp_hold[1], dp_cash[2] - p);
        let hold_0 = max(dp_hold[0], dp_cash[1] - p);

        dp_cash[2] = cash_2;
        dp_cash[1] = cash_1;
        dp_cash[0] = cash_0;
        dp_hold[1] = hold_1;
        dp_hold[0] = hold_0;
    }

    dp_cash.into_iter().max().unwrap()
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

    #[test]
    fn test_limit_2() {
        assert_eq!(0, stock_limit_2(vec![1]));
        assert_eq!(1, stock_limit_2(vec![1, 2]));
        assert_eq!(0, stock_limit_2(vec![2, 1]));
        assert_eq!(4, stock_limit_2(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, stock_limit_2(vec![7, 6, 4, 3, 1]));
        assert_eq!(6, stock_limit_2(vec![3, 3, 5, 0, 0, 3, 1, 4]));
    }
}
