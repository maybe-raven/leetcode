use std::collections::{hash_map::Iter, HashMap};

type Value = f64;
type Variable<'a> = &'a str;

trait DivisionExpression {
    /// Returns the variables in this expression as (top_variable, bottom_variable).
    fn unwrap(&self) -> (Variable, Variable);
}

impl DivisionExpression for Vec<String> {
    fn unwrap(&self) -> (Variable, Variable) {
        (self[0].as_str(), self[1].as_str())
    }
}

/// A collection that stores the edges a node has in a graph.
/// `N` is the type of the nodes in this graph.
/// `W` is the type of the weights of edges in this graph.
trait EdgeCollection {
    type Node;
    type Weight;
    type Iter: Iterator;

    fn insert(&mut self, target: Self::Node, weight: Self::Weight);
    fn get(&self, target: Self::Node) -> Self::Weight;
    fn iter(&self) -> Self::Iter;
}

impl<'a> EdgeCollection for HashMap<Variable<'a>, Value> {
    type Node = Variable<'a>;
    type Weight = Value;
    type Iter = Iter<'a, Self::Node, Self::Weight>;

    fn insert(&mut self, target: Self::Node, weight: Self::Weight) {
        todo!()
    }

    fn get(&self, target: Self::Node) -> Self::Weight {
        todo!()
    }

    fn iter(&self) -> Self::Iter {
        todo!()
    }
}

pub struct Solution {}

impl Solution {
    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        todo!()
    }
}
