type LinkedNodeIndex = Option<usize>;

struct LinkedNode<T> {
    value: T,
    next: LinkedNodeIndex,
}

struct GraphNode<T> {
    value: T,
    connection_head_index: LinkedNodeIndex,
}

struct Connection<W> {
    target_index: usize,
    weight: W,
}

pub struct Graph<T, W> {
    data: Vec<GraphNode<T>>,
    connections: Vec<LinkedNode<Connection<W>>>,
}
