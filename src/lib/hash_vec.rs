use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next_index: Option<usize>,
}

impl<T> Node<T> {
    fn with_value(value: T) -> Self {
        Self {
            value,
            next_index: None,
        }
    }

    #[allow(dead_code)]
    fn new(value: T, next_index: usize) -> Self {
        Self {
            value,
            next_index: Some(next_index),
        }
    }
}

impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.next_index == other.next_index
    }
}
impl<T: Eq> Eq for Node<T> {}

#[derive(Debug)]
pub struct NodeIter<'a, T> {
    next_index: Option<usize>,
    source: &'a Vec<Node<T>>,
}

impl<'a, T> NodeIter<'a, T> {
    fn new(source: &'a Vec<Node<T>>, first_index: usize) -> Self {
        Self {
            next_index: Some(first_index),
            source,
        }
    }
}

impl<'a, T> Iterator for NodeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_index.map(|index| {
            let node = &self.source[index];
            self.next_index = node.next_index;
            &node.value
        })
    }
}

#[derive(Debug, Default)]
pub struct HashVec<K, V> {
    data: Vec<Node<V>>,
    keys: HashMap<K, usize>,
}

impl<K, V> HashVec<K, V> {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            keys: HashMap::new(),
        }
    }
}

impl<K: Eq + Hash, V> HashVec<K, V> {
    pub fn insert(&mut self, key: K, value: V) {
        let mut node = Node::with_value(value);
        let new_index = self.data.len();

        self.keys
            .entry(key)
            .and_modify(|index| {
                node.next_index = Some(*index);
                *index = new_index;
            })
            .or_insert(new_index);
        self.data.push(node);
    }

    pub fn get(&self, key: &K) -> Option<NodeIter<V>> {
        self.keys
            .get(key)
            .map(|&index| NodeIter::new(&self.data, index))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert() {
        let mut hashvec: HashVec<i32, (i32, i32)> = HashVec::new();
        hashvec.insert(5, (0, 5));
        hashvec.insert(5, (1, 4));
        hashvec.insert(4, (2, 2));
        hashvec.insert(5, (2, 3));
        hashvec.insert(4, (1, 3));

        assert_eq!(hashvec.keys, HashMap::from([(5, 3), (4, 4)]));
        assert_eq!(
            hashvec.data,
            vec![
                Node::with_value((0, 5)),
                Node::new((1, 4), 0),
                Node::with_value((2, 2)),
                Node::new((2, 3), 1),
                Node::new((1, 3), 2),
            ]
        );
    }

    #[test]
    fn test_get() {
        let mut hashvec: HashVec<i32, (i32, i32)> = HashVec::new();
        hashvec.insert(5, (0, 5));
        hashvec.insert(5, (1, 4));
        hashvec.insert(4, (2, 2));
        hashvec.insert(5, (2, 3));
        hashvec.insert(4, (1, 3));

        let mut iter = hashvec.get(&5).expect("This key should exist.");
        assert_eq!(iter.next(), Some(&(2, 3)));
        assert_eq!(iter.next(), Some(&(1, 4)));
        assert_eq!(iter.next(), Some(&(0, 5)));
        assert_eq!(iter.next(), None);

        iter = hashvec.get(&4).expect("This key should exist.");
        assert_eq!(iter.next(), Some(&(1, 3)));
        assert_eq!(iter.next(), Some(&(2, 2)));
        assert_eq!(iter.next(), None);
    }
}
