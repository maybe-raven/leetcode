use std::collections::HashMap;

pub struct Solution {}

type Value = f64;
type VariableLabel<'a> = &'a str;

type VariableExpressionsType<'a> = HashMap<VariableLabel<'a>, Value>;

trait VariableExpressions<'a> {
    fn new() -> Self;
    fn add_expression(&mut self, variable_label: VariableLabel<'a>, multiplier: Value);
    fn get_multiplier(&self, variable_label: VariableLabel) -> Option<Value>;
}

impl<'a> VariableExpressions<'a> for VariableExpressionsType<'a> {
    fn new() -> Self {
        HashMap::new()
    }

    fn add_expression(&mut self, variable_label: VariableLabel<'a>, multiplier: Value) {
        self.insert(variable_label, multiplier);
    }

    fn get_multiplier(&self, variable_label: VariableLabel) -> Option<Value> {
        self.get(variable_label).copied()
    }
}

trait VariablePair {
    fn unpack(&self) -> (VariableLabel, VariableLabel);
}

impl VariablePair for Vec<String> {
    fn unpack(&self) -> (VariableLabel, VariableLabel) {
        (self[0].as_str(), self[1].as_str())
    }
}

type VariableMapType<'a> = HashMap<VariableLabel<'a>, VariableExpressionsType<'a>>;
trait VariableMap<'a> {
    fn process_equation<T: VariablePair>(&mut self, equation: (&'a T, Value));
    fn get_expressions(&self, variable: VariableLabel) -> Option<&VariableExpressionsType>;

    fn get_value_of(
        &self,
        variable: VariableLabel,
        as_multiples_of: VariableLabel,
    ) -> Option<Value> {
        self.get_expressions(variable).and_then(|expr| {
            expr.get_multiplier(as_multiples_of).or_else(|| {
                expr.iter().find_map(|(&key, &multiplier_0)| {
                    self.get_value_of(key, as_multiples_of)
                        .map(|multiplier_1| multiplier_0 * multiplier_1)
                })
            })
        })
    }

    fn get_value<T: VariablePair>(&self, variable_pair: &T) -> Option<Value> {
        let (top_variable, bottom_variable) = variable_pair.unpack();
        self.get_value_of(top_variable, bottom_variable)
    }
}

impl<'a> VariableMap<'a> for VariableMapType<'a> {
    fn process_equation<T: VariablePair>(&mut self, (variable_pair, value): (&'a T, Value)) {
        let (top_variable, bottom_variable) = variable_pair.unpack();

        self.entry(top_variable)
            .or_insert_with(VariableExpressionsType::new)
            .add_expression(bottom_variable, value);

        self.entry(bottom_variable)
            .or_insert_with(VariableExpressionsType::new)
            .add_expression(top_variable, 1.0 / value);
    }

    fn get_expressions(&self, variable: VariableLabel) -> Option<&VariableExpressionsType> {
        self.get(variable)
    }
}

trait SolutionResult {
    fn value(self) -> Value;
}

impl SolutionResult for Option<Value> {
    fn value(self) -> Value {
        self.unwrap_or(-1.0)
    }
}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut var_map = VariableMapType::new();
        for item in equations.iter().zip(values) {
            var_map.process_equation(item)
        }

        queries
            .iter()
            .map(|pair| var_map.get_value(pair).value())
            .collect()
    }
}
