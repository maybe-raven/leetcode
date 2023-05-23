use std::collections::{HashMap, HashSet};

pub struct Solution {}

fn get_value(
    graph: &HashMap<&str, HashMap<&str, f64>>,
    variable_pair: &Vec<String>,
) -> Option<f64> {
    let top_variable = variable_pair[0].as_str();
    let bottom_variable = variable_pair[1].as_str();

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
    let edges = graph
        .get(start)
        .expect("Both `start` and `end` should be in `graph`.");

    if let Some(&result) = edges.get(end) {
        return Some(result);
    }

    for (&next, &weight) in edges {
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
        for (variable_pair, value) in equations.iter().zip(values) {
            let top_variable = variable_pair[0].as_str();
            let bottom_variable = variable_pair[1].as_str();

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
            .map(|pair| get_value(&graph, pair).unwrap_or(-1.0))
            .collect()
    }
}
