use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

/// Represent the weight between a node and itself in a graph.
pub trait One: Sized {
    fn one() -> Option<Self>;
}

/// Combine two weights. Used to calculate the total weight between two nodes.
pub trait Combine {
    fn combine(&self, other: &Self) -> Self;
}

/// Represent a directional connection in a graph as `ConnectionTo(target, weight)`.
pub struct ConnectionTo<Node, Weight>(Node, Weight);

impl<Node, Weight> ConnectionTo<Node, Weight> {
    pub fn new(target: Node, weight: Weight) -> Self {
        Self(target, weight)
    }
}

/// A collection of outgoing directional connections of a node in a graph.
pub trait ConnectionCollection<Node, Weight> {
    fn add_connection(&mut self, connection: ConnectionTo<Node, Weight>);
    fn get_weight_to(&self, target: &Node) -> Option<&Weight>;
}

/// A graph with directional connections.
/// `Connections` is the type used to manage the outgoing connections a node has.
pub trait Graph<Node, Weight, Connections> {
    fn add_connection(&mut self, node: Node, connection: ConnectionTo<Node, Weight>);
    fn get_connections_from(&self, node: &Node) -> Option<&Connections>;

    fn has_node(&self, node: &Node) -> bool {
        self.get_connections_from(node).is_some()
    }
}

/// A trait to add functions to some types that already implement `Graph`.
/// Type parameters are the same as `Graph`, just with more specific bounds.
pub trait GraphSolver<'a, Node, Weight, Connections>: Graph<Node, Weight, Connections>
where
    Node: Eq + Hash + 'a,
    Weight: Copy + Combine + One + 'a,
    Connections: ConnectionCollection<Node, Weight> + 'a,
    &'a Connections: IntoIterator,
    <&'a Connections as IntoIterator>::Item: Into<ConnectionTo<&'a Node, &'a Weight>>,
{
    /// Find the weight from the `start` node to the `end` node.
    fn find_total_weight(&'a self, start: &'a Node, end: &Node) -> Option<Weight> {
        if !self.has_node(start) || !self.has_node(end) {
            return None;
        }

        if start == end {
            return Weight::one();
        }

        let mut visited = HashSet::new();
        visited.insert(start);
        self.find_total_weight_recursively(start, end, &mut visited)
    }

    /// Internal function. Don't call this.
    /// Find *a* path from `start` to `end` recursively using DFS.
    /// Returns the total weight of all the connections between `start` and `end`.
    fn find_total_weight_recursively(
        &'a self,
        start: &Node,
        end: &Node,
        visited: &mut HashSet<&'a Node>,
    ) -> Option<Weight> {
        // <Self as Graph<Node, Weight, Connections>>::get_connections_from(self: &'b Self, start: &'c Node) -> Option<&'b Connections>;
        // edges: &'b Connections;
        let edges = self
            .get_connections_from(start)
            .expect("Both `start` and `end` are expected to be present in the graph.");

        // <Connections as ConnectionCollection>::get_weight_to(edges: &'b Connections, end: &'d Node) -> Option<&'b Weight>;
        // result: Option<&'b Weight>;
        let result = edges.get_weight_to(end);
        if result.is_some() {
            return result.copied();
        }

        // <&'b Connections as IntoIterator>::into_iter(edges: &'b Connections) -> Iter;
        // where Iter::Item: Into<ConnectionTo<&'b Node, &'b Weight>>
        for item in edges.into_iter() {
            // next: &'b Node;
            // weight: &'b Weight;
            let ConnectionTo(next, weight) = item.into();

            // &'b Node: &'f Node
            // 'b: 'f
            // HashSet::<&'f Node>::insert(&'e mut <&'f Node>, &'f Node) -> bool;
            // HashSet::<&Node>::insert(visited, next);
            if visited.insert(next) {
                if let Some(result) = self.find_total_weight_recursively(next, end, visited) {
                    return Some(result.combine(weight));
                }
            }
        }

        None
    }
}

/// Automatically implement this trait for all types that already implements `Graph`
/// and satisfy these additional constraints which are required for `GraphSolver` to work.
impl<'a, Node, Weight, Connections, G> GraphSolver<'a, Node, Weight, Connections> for G
where
    Node: Eq + Hash + 'a,
    Weight: Copy + Combine + One + 'a,
    Connections: ConnectionCollection<Node, Weight> + 'a,
    &'a Connections: IntoIterator,
    <&'a Connections as IntoIterator>::Item: Into<ConnectionTo<&'a Node, &'a Weight>>,
    G: Graph<Node, Weight, Connections>,
{
}

/// Implement conversion between `ConnectionTo` and `<HashMap as IntoIterator>::Item`.
impl<N, W> From<(N, W)> for ConnectionTo<N, W> {
    fn from((n, w): (N, W)) -> Self {
        Self(n, w)
    }
}

/// Generic `ConnectionCollection` implement using `HashMap`.
impl<N: Eq + Hash, W> ConnectionCollection<N, W> for HashMap<N, W> {
    fn add_connection(&mut self, ConnectionTo(target, weight): ConnectionTo<N, W>) {
        self.insert(target, weight);
    }

    fn get_weight_to(&self, target: &N) -> Option<&W> {
        self.get(target)
    }
}

/// Generic `Graph` implement using `HashMap`.
impl<'a, Node, Weight, Connections> Graph<Node, Weight, Connections> for HashMap<Node, Connections>
where
    Node: Eq + Hash + 'a,
    Weight: One + 'a,
    Connections: Default + ConnectionCollection<Node, Weight> + 'a,
    &'a Connections: IntoIterator,
    <&'a Connections as IntoIterator>::Item: Into<ConnectionTo<&'a Node, &'a Weight>>,
{
    fn add_connection(&mut self, node: Node, edge: ConnectionTo<Node, Weight>) {
        self.entry(node).or_default().add_connection(edge);
    }

    fn get_connections_from(&self, node: &Node) -> Option<&Connections> {
        self.get(node)
    }

    fn has_node(&self, node: &Node) -> bool {
        self.contains_key(node)
    }
}

/// Implement conversion between `ConnectionTo` and `<Vec as IntoIterator>::Item`.
impl<'a, N, W> From<&'a (N, W)> for ConnectionTo<&'a N, &'a W> {
    fn from((n, w): &'a (N, W)) -> Self {
        ConnectionTo(n, w)
    }
}

/// Generic `ConnectionCollection` implement using `HashMap`.
impl<N: Eq, W> ConnectionCollection<N, W> for Vec<ConnectionTo<N, W>> {
    fn add_connection(&mut self, edge: ConnectionTo<N, W>) {
        self.push(edge);
    }

    fn get_weight_to(&self, target: &N) -> Option<&W> {
        for ConnectionTo(node, weight) in self {
            if node == target {
                return Some(weight);
            }
        }

        return None;
    }
}

/// Generic `Graph` implement using `Vec`.
impl<'a, Node, Weight, Connections> Graph<Node, Weight, Connections> for Vec<(Node, Connections)>
where
    Node: Eq + 'a,
    Weight: One + 'a,
    Connections: Default + ConnectionCollection<Node, Weight> + 'a,
    &'a Connections: IntoIterator,
    <&'a Connections as IntoIterator>::Item: Into<ConnectionTo<&'a Node, &'a Weight>>,
{
    fn add_connection(&mut self, target: Node, connection: ConnectionTo<Node, Weight>) {
        for (node, connections) in self.iter_mut() {
            if *node == target {
                connections.add_connection(connection);
                return;
            }
        }
        let mut connections = Connections::default();
        connections.add_connection(connection);
        self.push((target, connections));
    }

    fn get_connections_from(&self, target: &Node) -> Option<&Connections> {
        for (node, connections) in self {
            if node == target {
                return Some(connections);
            }
        }
        None
    }
}
