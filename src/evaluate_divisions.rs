struct Solution {}

use std::collections::{btree_map::Keys, HashMap};

type Value = f64;
type VariableLabel = String;

/// A collection of expressions that a particular variable is equivalent to.
/// Expressions are given in the form of `multiplier * variable_label`.
trait VariableExpressions<'a> {
    fn new() -> Self;
    /// Finds a variable that is common to both sets of expressions.
    fn find_common(lhs: &Self, rhs: &Self) -> Option<&'a String>;
    fn add_expression(&mut self, variable_label: &'a String, multiplier: Value);
    fn get_multiplier(&self, variable_label: &String) -> Option<Value>;

    fn calc_result_with_common_key(
        top_expressions: &Self,
        bottom_expressions: &Self,
        common_key: &String,
    ) -> Value {
        top_expressions.get_multiplier(common_key).unwrap()
            / bottom_expressions.get_multiplier(common_key).unwrap()
    }

    fn find_value_by_common_key(
        top_expressions: &Self,
        bottom_expressions: &Self,
    ) -> Option<Value> {
        Self::find_common(top_expressions, bottom_expressions).and_then(|common_key| {
            Some(Self::calc_result_with_common_key(
                top_expressions,
                bottom_expressions,
                common_key,
            ))
        })
    }
    fn find_value(
        top_expressions: &Self,
        bottom_expressions: &Self,
        bottom_label: &String,
    ) -> Option<Value> {
        top_expressions
            .get_multiplier(bottom_label)
            .or_else(|| Self::find_value_by_common_key(top_expressions, bottom_expressions))
    }
}

impl<'a> VariableExpressions<'a> for HashMap<&'a String, Value> {
    fn new() -> Self {
        Self::new()
    }
    fn add_expression(&mut self, variable_label: &'a String, multiplier: Value) {
        self.insert(variable_label, multiplier);
    }

    fn get_multiplier(&self, variable_label: &String) -> Option<Value> {
        self.get(variable_label).copied()
    }

    fn find_common(lhs: &Self, rhs: &Self) -> Option<&'a String> {
        for key in lhs.keys() {
            if rhs.contains_key(key) {
                return Some(key);
            }
        }
        return None;
    }
}

trait VariablePair {
    fn unpack(&self) -> (&String, &String);
}

impl VariablePair for Vec<String> {
    fn unpack(&self) -> (&String, &String) {
        (&self[0], &self[1])
    }
}

trait VariableMap<'a, T: VariablePair, V: VariableExpressions<'a>> {
    fn add_item(&mut self, item: (&'a T, Value));
    fn get_expressions(&self, variable: &String) -> Option<&V>;
    fn find_value(&self, variable_pair: &'a T) -> Option<Value>;

    fn get_value_of(&self, variable: &String, as_multiples_of: &String) -> Option<Value> {
        self.get_expressions(variable)
            .and_then(|expr| expr.get_multiplier(as_multiples_of))
    }
}

impl<'a, T: VariablePair, V: VariableExpressions<'a>> VariableMap<'a, T, V>
    for HashMap<&'a String, V>
{
    fn add_item(&mut self, (var_labels, value): (&'a T, Value)) {
        let (var_a, var_b) = var_labels.unpack();

        self.entry(var_a)
            .or_insert_with(V::new)
            .add_expression(var_b, value);

        self.entry(var_b)
            .or_insert_with(V::new)
            .add_expression(var_a, 1.0 / value);
    }

    fn get_expressions(&self, variable: &String) -> Option<&V> {
        self.get(variable)
    }

    fn find_value(&self, variable_pair: &'a T) -> Option<Value> {
        let (var_a, var_b) = variable_pair.unpack();
        if let (Some(expr_a), Some(expr_b)) = (self.get(var_a), self.get(var_b)) {
            V::find_value(expr_a, expr_b, var_b)
        } else {
            None
        }
    }
}

trait SolutionResult {
    fn value(self) -> Value;
}

impl SolutionResult for Option<Value> {
    fn value(self) -> Value {
        match self {
            None => -1.0,
            Some(result) => result,
        }
    }
}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut var_map = HashMap::<&String, HashMap<&String, Value>>::new();
        for item in equations.iter().zip(values) {
            var_map.add_item(item);
        }

        queries
            .iter()
            .map(|pair| var_map.find_value(pair).value())
            .collect()
    }
}
