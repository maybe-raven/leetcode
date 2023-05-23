use std::{
    collections::{
        hash_map::{Iter, Keys},
        HashMap,
    },
    hash::Hash,
};

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

trait Edge<N, W> {
    fn target(&self) -> &N;
    fn weight(&self) -> &W;
    fn unwrap(self) -> (N, W);
}

impl<N, W> Edge<N, W> for (N, W) {
    fn target(&self) -> &N {
        &self.0
    }

    fn weight(&self) -> &W {
        &self.1
    }

    fn unwrap(self) -> (N, W) {
        self
    }
}

impl<N: Copy, W: Copy> Edge<N, W> for (&N, &W) {
    fn target(&self) -> &N {
        self.0
    }

    fn weight(&self) -> &W {
        self.1
    }

    fn unwrap(self) -> (N, W) {
        (*self.0, *self.1)
    }
}

impl<N: Copy, W: Copy> Edge<N, W> for &(N, W) {
    fn target(&self) -> &N {
        &self.0
    }

    fn weight(&self) -> &W {
        &self.1
    }

    fn unwrap(self) -> (N, W) {
        *self
    }
}

/// A collection that stores the edges a node has in a graph.
/// `N` is the type of the nodes in this graph.
/// `W` is the type of the weights of edges in this graph.
trait EdgeCollection<N, W, I>: Default
where
    I: Iterator,
    I::Item: Edge<N, W>,
{
    fn insert_edge<E: Edge<N, W>>(&mut self, edge: E);
    fn get_edge_to(&self, target: &N) -> Option<&W>;
    fn iter(&self) -> I;
}
impl<N, W> EdgeCollection<N, W, Iter<'_, N, W>> for HashMap<N, W>
where
    N: Eq + Hash + Copy,
    W: Copy,
{
    fn insert_edge<E: Edge<N, W>>(&mut self, edge: E) {
        let (target, weight) = edge.unwrap();
        self.insert(target, weight);
    }

    fn get_edge_to(&self, target: &N) -> Option<&W> {
        self.get(target)
    }

    fn iter(&self) -> Iter<'_, N, W> {
        HashMap::iter(self)
    }
}
// impl<N, W> EdgeCollection<N, W> for HashMap<N, W> {
//     type Item = (&'a N, &'a W);
//     type Iter = Keys<'a, N, W>;

//     fn insert_edge<E: Edge<N, W>>(&mut self, edge: E) {
//         let (target, weight) = edge.unwrap();
//         self.insert(target, weight);
//     }

//     fn get_edge_to(&self, target: &N) -> Option<&W> {
//         self.get(target)
//     }

//     fn iter(&self) -> Self::Iter {
//         HashMap::iter(self)
//     }
// }

// impl<'a, N: Eq + Hash + 'a, W: 'a> EdgeCollection<'a, N, W> for HashMap<N, W> {
//     type Iter = Iter<'a, N, W>;

//     fn insert_edge_to(&mut self, target: N, weight: W) {
//         self.insert(target, weight);
//     }

//     fn get_edge_to(&self, target: &N) -> Option<&W> {
//         self.get(target)
//     }

//     fn iter(&'a self) -> Self::Iter {
//         HashMap::iter(self)
//     }
// }

// impl<'a, N: Eq + 'a, W: 'a> EdgeCollection<'a, N, W> for Vec<(N, W)> {
//     type Iter;

//     fn insert_edge_to(&mut self, target: N, weight: W) {
//         todo!()
//     }

//     fn get_edge_to(&self, target: &N) -> Option<&W> {
//         todo!()
//     }

//     fn iter(&'a self) -> Self::Iter {
//         Vec::iter(self)
//     }
// }

// impl<K, V> EdgeCollection for HashMap<K, V> {
//     type Node = K;
//     type Weight = V;

//     fn insert_edge_to(&mut self, target: Self::Node, weight: Self::Weight) {
//         todo!()
//     }

//     fn get_edge_to(&self, target: Self::Node) -> Self::Weight {
//         todo!()
//     }

//     fn iter<'a, T: Iterator<Item = (&'a Self::Node, &'a Self::Weight)>>(&'a self) -> T {
//         todo!()
//     }
// }

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
