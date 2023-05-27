//! https://leetcode.com/problems/stone-game-ii/

// let sums[i] be the score a player can get
// by taking the i'th pile and all the piles after it.
// it is also the total score available to both players from here on,
// given i piles have already been taken.
// i.e. let sums[i] be piles[i..].sum().

// let dp[i][m] be the maximum score a player can get from here on,
// if they start their turn given i and m, where
// i is the number of piles already taken,
// and m is m from the question description,
// which is half of the maximum number of piles they can take on that turn.

// since this game is zero sum,
// a player's maximum score from a particular turn onward,
// is the total available score from that turn onward
// minus their opponent's maximum score from that turn onward.

// therefore, given i piles already taken and m at the start of this turn,
// and x piles being taken this turn,
// then they gain sums[i] - sums[i + x] stones this turn,
// and sums[i + x] - dp[i + x][max(m, x)] stones from next turn onward,
// so the total score from this turn onward is
// sums[i] - sums[i + x] + sums[i + x] - dp[i + x][max(m, x)]
// = sums[i] - dp[i + x][max(m, x)]

use std::cmp::max;

fn convert_index(i: usize, m: usize, n: usize) -> usize {
    m * n + i
}

fn dps(sums: &Vec<i32>, dp: &mut Vec<Option<i32>>, i: usize, m: usize) -> i32 {
    let n = sums.len();

    if i + 2 * m >= n {
        return sums[i];
    }

    if let Some(result) = dp[convert_index(i, m, n)] {
        return result;
    }

    let result = (1..=2 * m)
        .map(|x| sums[i] - dps(sums, dp, i + x, max(m, x)))
        .max()
        .unwrap();

    dp[convert_index(i, m, n)] = Some(result);

    result
}

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let mut sums = piles;

        let mut acc = 0;
        for x in sums.iter_mut().rev() {
            acc += *x;
            *x = acc;
        }

        let max_m = (sums.len() as f32 / 2.0).ceil() as usize;

        let mut dp: Vec<Option<i32>> = vec![None; sums.len() * max_m];

        dps(&sums, &mut dp, 0, 1)
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {

    use std::iter::repeat;

    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(Solution::stone_game_ii(vec![3]), 3);
        assert_eq!(Solution::stone_game_ii(vec![1, 1, 1]), 2);
        assert_eq!(Solution::stone_game_ii(vec![2, 7, 9, 4, 4]), 10);
        assert_eq!(Solution::stone_game_ii(vec![1, 2, 3, 4, 5, 100]), 104);
        assert_eq!(Solution::stone_game_ii((1..=20).collect()), 108);
        let input = repeat([9, 5, 1, 2, 4, 3, 7, 8, 6])
            .flatten()
            .take(100)
            .collect();
        assert_eq!(Solution::stone_game_ii(input), 260);
    }
}
