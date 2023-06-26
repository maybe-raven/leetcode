//! 2462. Total Cost to Hire K Workers
//! https://leetcode.com/problems/total-cost-to-hire-k-workers

impl Solution {
    pub fn total_cost(costs: Vec<i32>, k: i32, candidates: i32) -> i64 {
        assert!(1 <= k && k as usize <= costs.len());
        assert!(1 <= candidates && candidates as usize <= costs.len());
        assert!(1 <= costs.len());

        let mut indices: Vec<Option<usize>> = (0..costs.len()).map(Some).collect();
        indices.sort_by_key(|&i| costs[i.unwrap()]);

        let mut lc = candidates as usize;
        let mut rc = costs.len() - lc;
        let mut total = 0;
        let mut n = 0;
        let mut should_restart = false;

        loop {
            for c in indices.iter_mut() {
                if n == k {
                    return total;
                }

                let &mut Some(cv) = c else { continue; };

                if lc <= cv && cv < rc {
                    should_restart = true;
                    continue;
                }

                if cv < lc {
                    lc += 1;
                } else if cv >= rc {
                    rc -= 1;
                }

                total += costs[cv] as i64;
                n += 1;
                *c = None;

                if should_restart {
                    break;
                }
            }
        }
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(
            11,
            Solution::total_cost(vec![17, 12, 10, 2, 7, 2, 11, 20, 8], 3, 4)
        );
        assert_eq!(4, Solution::total_cost(vec![1, 2, 4, 1], 3, 3));
        assert_eq!(
            106,
            Solution::total_cost(
                vec![100, 99, 98, 97, 1, 2, 3, 4, 5, 6, 7, 8, 80, 81, 82, 83],
                5,
                4
            )
        );
    }
}
