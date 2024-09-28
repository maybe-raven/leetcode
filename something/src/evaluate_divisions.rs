use std::{collections::HashMap, hash::Hash, ops::Div};

use crate::graph::{Combine, ConnectionTo, Graph, GraphSolver, One};

#[derive(Debug, Clone, Copy)]
struct Value(f64);

impl Div<Value> for f64 {
    type Output = Value;

    fn div(self, rhs: Value) -> Self::Output {
        Value(self * rhs.0)
    }
}

impl Combine for Value {
    fn combine(&self, other: &Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl One for Value {
    fn one() -> Option<Self> {
        Some(Self(1.0))
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self(value)
    }
}

impl From<Value> for f64 {
    fn from(value: Value) -> Self {
        value.0
    }
}

impl From<Option<Value>> for Value {
    fn from(value: Option<Value>) -> Self {
        if let Some(result) = value {
            result
        } else {
            Self(-1.0)
        }
    }
}

// #[derive(Debug, Clone, Copy)]
// struct Result(Option<Value>);

// impl Mul<Self> for Result {
//     type Output = Self;

//     fn mul(self, rhs: Self) -> Self::Output {
//         if let (Some(lhs), Some(rhs)) = (self.0, rhs.0) {
//             Self::from(lhs * rhs)
//         } else {
//             Self(None)
//         }
//     }
// }

// impl Mul<Value> for Result {
//     type Output = Self;

//     fn mul(self, rhs: Value) -> Self::Output {
//         if let Some(lhs) = self.0 {
//             Self::from(lhs * rhs)
//         } else {
//             Self(None)
//         }
//     }
// }

// impl From<Value> for Result {
//     fn from(value: Value) -> Self {
//         Self(Some(value))
//     }
// }

// impl From<&Value> for Result {
//     fn from(value: &Value) -> Self {
//         Self::from(*value)
//     }
// }

// impl From<Result> for Value {
//     fn from(value: Result) -> Self {
//         value.0.unwrap_or(-1.0)
//     }
// }

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Variable<'a>(&'a str);

impl<'a> From<&'a String> for Variable<'a> {
    fn from(value: &'a String) -> Self {
        Self(&value.as_str())
    }
}

impl<'a> From<Variable<'a>> for &'a str {
    fn from(value: Variable<'a>) -> Self {
        value.0
    }
}

/// A division of `a / b` stored as `DivisionExpression(a, b)`
#[derive(Debug)]
struct DivisionExpression<'a>(Variable<'a>, Variable<'a>);

// impl<'a> DivisionExpression<'a> {
//     fn top_variable(&self) -> Variable {
//         self.0
//     }
//     fn bottom_variable(&self) -> Variable {
//         self.1
//     }
// }

impl<'a> From<&'a Vec<String>> for DivisionExpression<'a> {
    fn from(variable_pair: &'a Vec<String>) -> Self {
        Self(
            Variable::from(&variable_pair[0]),
            Variable::from(&variable_pair[1]),
        )
    }
}
/// A division of `a / b = v` stored as `DivisionEquation(DivisionExpression(a, b), v)`
#[derive(Debug)]
struct DivisionEquation<'a>(DivisionExpression<'a>, Value);

impl<'a, T: Into<DivisionExpression<'a>>, V: Into<Value>> From<(T, V)> for DivisionEquation<'a> {
    fn from((expr, value): (T, V)) -> Self {
        Self(expr.into(), value.into())
    }
}

pub struct Solution;

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        let mut graph = HashMap::<Variable, HashMap<Variable, Value>>::new();
        for item in equations.iter().zip(values) {
            let DivisionEquation(DivisionExpression(top_variable, bottom_variable), value) =
                item.into();

            graph.add_connection(top_variable, ConnectionTo::new(bottom_variable, value));
            graph.add_connection(
                bottom_variable,
                ConnectionTo::new(top_variable, 1.0 / value),
            );
        }

        queries
            .iter()
            .map(|query| -> f64 {
                let DivisionExpression(top_variable, bottom_variable) = query.into();
                Into::<Value>::into(graph.find_total_weight(&top_variable, &bottom_variable)).into()
            })
            .collect()
    }
}
