//! 714. Best Time to Buy and Sell Stock with Transaction Fee
//! https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee

use std::cmp::Ordering;

trait IsNoneOr<T>: Copy {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T: Copy> IsNoneOr<T> for Option<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            Some(x) => f(x),
            None => true,
        }
    }
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let mut memo: Vec<(i32, Option<i32>, Option<i32>)> = vec![(0, None, None)];
        let mut extender: Vec<(i32, Option<i32>, Option<i32>)> = Vec::new();

        // println!("{:?}", &prices);
        for &price in &prices {
            for (profit, buy_price, sell_price) in memo.iter_mut() {
                let buy = buy_price.get_or_insert(price);
                if price < *buy {
                    if let Some(sell) = sell_price {
                        *profit += *sell - *buy - fee;
                        *sell_price = None;
                    }
                    *buy = price;
                } else if price > *buy + fee && sell_price.is_none_or(|x| price > x) {
                    *sell_price = Some(price);
                } else if let Some(sell) = sell_price {
                    extender.push(((*profit + *sell - *buy - fee), Some(price), None));
                }
            }

            memo.append(&mut extender);
            memo.sort_unstable_by(|(p0, b0, s0), (p1, b1, s1)| match b0.cmp(b1) {
                // Buy price in increasing order
                Ordering::Equal => match s1.cmp(s0) {
                    // Sell price in decreasing order.
                    Ordering::Equal => p1.cmp(p0), // Profit in decreasing order.
                    ordering => ordering,
                },
                ordering => ordering,
            });
            memo.dedup_by_key(|&mut (_, buy_price, sell_price)| (buy_price, sell_price));

            // println!("price: {}", price);
            // for (profit, buy_price, sell_price) in memo.iter() {
            //     println!(
            //         "profit: {}; buy: {:?}; sell: {:?}",
            //         profit, buy_price, sell_price
            //     );
            // }
        }

        memo.into_iter()
            .map(|(profit, buy_price, sell_price)| {
                if let (Some(buy), Some(sell)) = (buy_price, sell_price) {
                    profit + sell - buy - fee
                } else {
                    profit
                }
            })
            .max()
            .unwrap()
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
