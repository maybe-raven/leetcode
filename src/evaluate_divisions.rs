//! https://leetcode.com/problems/evaluate-division/

use std::collections::{HashMap, HashSet};

use crate::return_if_some;

fn get_value(
    graph: &HashMap<&str, HashMap<&str, f64>>,
    division_expression: &[String],
) -> Option<f64> {
    let top_variable = division_expression[0].as_str();
    let bottom_variable = division_expression[1].as_str();

    if !graph.contains_key(top_variable) || !graph.contains_key(bottom_variable) {
        return None;
    }

    if top_variable == bottom_variable {
        return Some(1.0);
    }

    let mut visited = HashSet::new();
    dfs(graph, &mut visited, top_variable, bottom_variable)
}

fn dfs<'a>(
    graph: &HashMap<&str, HashMap<&'a str, f64>>,
    visited: &mut HashSet<&'a str>,
    start: &str,
    end: &str,
) -> Option<f64> {
    let connections = graph.get(start)?;

    return_if_some! { connections.get(end).copied() }

    for (&next, &weight) in connections {
        if visited.contains(next) {
            continue;
        }

        visited.insert(next);
        if let Some(result) = dfs(graph, visited, next, end) {
            return Some(result * weight);
        }
    }

    None
}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph = HashMap::new();
        for (division_expression, value) in equations.iter().zip(values) {
            let top_variable = division_expression[0].as_str();
            let bottom_variable = division_expression[1].as_str();

            graph
                .entry(top_variable)
                .or_insert_with(HashMap::new)
                .insert(bottom_variable, value);
            graph
                .entry(bottom_variable)
                .or_insert_with(HashMap::new)
                .insert(top_variable, 1.0 / value);
        }

        queries
            .iter()
            .map(|query| get_value(&graph, query).unwrap_or(-1.0))
            .collect()
    }
}

pub struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_0() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
        ];
        let values = vec![2.0, 3.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["b".to_string(), "a".to_string()],
            vec!["a".to_string(), "e".to_string()],
            vec!["a".to_string(), "a".to_string()],
            vec!["x".to_string(), "x".to_string()],
        ];
        let expected = vec![6.00000, 0.50000, -1.00000, 1.00000, -1.00000];
        assert_eq!(
            Solution::calc_equation(equations, values, queries),
            expected
        );
    }
    #[test]
    fn test_case_1() {
        let equations = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "c".to_string()],
            vec!["bc".to_string(), "cd".to_string()],
        ];
        let values = vec![1.5, 2.5, 5.0];
        let queries = vec![
            vec!["a".to_string(), "c".to_string()],
            vec!["c".to_string(), "b".to_string()],
            vec!["bc".to_string(), "cd".to_string()],
            vec!["cd".to_string(), "bc".to_string()],
        ];
        let expected = vec![3.75000, 0.40000, 5.00000, 0.20000];
        assert_eq!(
            Solution::calc_equation(equations, values, queries),
            expected
        );
    }
    #[test]
    fn test_case_2() {
        let equations = vec![vec!["a".to_string(), "b".to_string()]];
        let values = vec![0.5];
        let queries = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["b".to_string(), "a".to_string()],
            vec!["a".to_string(), "c".to_string()],
            vec!["x".to_string(), "y".to_string()],
        ];
        let expected = vec![0.50000, 2.00000, -1.00000, -1.00000];
        assert_eq!(
            Solution::calc_equation(equations, values, queries),
            expected
        );
    }
}
