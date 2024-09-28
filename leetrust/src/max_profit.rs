//! 714. Best Time to Buy and Sell Stock with Transaction Fee
//! https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut profit = 0;
        let mut buy_price = None;
        let mut sell_price: Option<i32> = None;

        for &price in &prices {
            let buy = buy_price.get_or_insert(price);
            if price < *buy {
                if let Some(sell) = sell_price {
                    profit += sell - *buy - fee;
                    sell_price = None;
                }
                *buy = price;
            } else if let Some(sell) = sell_price {
                if price > sell {
                    sell_price = Some(price);
                    continue;
                } else if sell > price + fee {
                    profit += sell - *buy - fee;
                    *buy = price;
                    sell_price = None;
                }
            } else if price > *buy + fee {
                sell_price = Some(price);
            }
        }

        if let (Some(buy), Some(sell)) = (buy_price, sell_price) {
            profit + sell - buy - fee
        } else {
            profit
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(8, Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2));
        assert_eq!(6, Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3));
        assert_eq!(
            6,
            Solution::max_profit(
                vec![1, 2, 3, 4, 3, 2, 1, 5, 6, 5, 7, 5, 8, 5, 9, 10, 2, 3],
                3
            )
        );
        assert_eq!(
            9,
            Solution::max_profit(
                vec![1, 2, 3, 4, 3, 2, 1, 5, 6, 5, 7, 5, 8, 5, 9, 10, 2, 3],
                2
            )
        );
        assert_eq!(
            16,
            Solution::max_profit(vec![1, 10, 2, 9, 3, 8, 4, 7, 5, 6], 2)
        );
    }
}
