//! 1575. Count All Possible Routes
//! https://leetcode.com/problems/count-all-possible-routes

const MOD: i32 = 1000000007;

impl Solution {
    pub fn count_routes(locations: Vec<i32>, start: i32, finish: i32, fuel: i32) -> i32 {
        let start = start as usize;
        let finish = finish as usize;
        let fuel = fuel as usize;

        let mut memo: Vec<Vec<i32>> = (0..locations.len())
            .map(|i| vec![if i == finish { 1 } else { 0 }; fuel + 1])
            .collect();

        let fuel_costs_table: Vec<Vec<i32>> = locations
            .iter()
            .map(|x| locations.iter().map(|y| (x - y).abs()).collect())
            .collect();

        for f in 1..=fuel {
            let fi = f as i32;
            for (i, costs) in fuel_costs_table.iter().enumerate() {
                let mut n = 0;
                for (j, &cost) in costs.iter().enumerate() {
                    if let Ok(cost) = usize::try_from(fi - cost) {
                        n += memo[j][cost];
                        n %= MOD;
                    }
                }
                memo[i][f] = n;
            }
        }

        memo[start][fuel]
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(4, Solution::count_routes(vec![2, 3, 6, 8, 4], 1, 3, 5));
        assert_eq!(5, Solution::count_routes(vec![4, 3, 1], 1, 0, 6));
        assert_eq!(0, Solution::count_routes(vec![5, 2, 1], 0, 2, 3));
        assert_eq!(8, Solution::count_routes(vec![1, 2, 3, 4, 5], 0, 4, 5));
    }
}
