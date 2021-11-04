//!
//! # Chapter 2 Utilities and Definitions
//!

use std::ops::{Index, IndexMut};

/// Node Index
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NodeIndex(pub usize);

/// Doubly Linked List Node
#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub index: NodeIndex,
    pub next: Option<NodeIndex>,
    pub prev: Option<NodeIndex>,
}

/// Doubly Linked List
#[derive(Debug, Default)]
pub struct List<T: Default> {
    pub nodes: Vec<Node<T>>,
    pub head: Option<NodeIndex>,
    pub tail: Option<NodeIndex>,
}
impl<T: Clone + Default> List<T> {
    /// Also available via [Default]
    pub fn new() -> Self {
        Self::default()
    }
    /// Push a new node with `data` to the end of the list.
    pub fn push(&mut self, data: T) {
        // Create the new node with `data`
        let node = Node {
            data,
            index: NodeIndex(self.nodes.len()),
            next: None,
            prev: self.tail,
        };
        // Update the next-pointer on the current tail-node
        if let Some(tail) = self.tail {
            self[tail].next = Some(node.index);
        }
        // Update the tail-node to be this new node
        self.tail = Some(node.index);
        // If necessary, update the head-node
        if self.head.is_none() {
            self.head = Some(node.index);
        }
        // And finally give our internal [Vec] ownership of the node
        self.nodes.push(node);
    }
    /// Convert to a vector of `T` data elements, cloning each along the way.
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::new();
        let mut node = self.head;
        while let Some(idx) = node {
            vec.push(self[idx].data.clone());
            node = self[idx].next;
        }
        vec
    }
    /// Remove the node at `index` from the list.
    /// Removed nodes *are not* deallocated, just removed from the pointer-chain.
    pub fn remove(&mut self, idx: NodeIndex) {
        // Update the previous node's next pointer.
        if let Some(prev) = self[idx].prev {
            self[prev].next = self[idx].next;
        } else {
            self.head = self[idx].next;
        }
        // Update the next node's prev pointer.
        if let Some(next) = self[idx].next {
            self[next].prev = self[idx].prev;
        } else {
            self.tail = self[idx].prev;
        }
    }
}
/// Create a [List] from a vector or similar slice of `T` data elements.
impl<T: Clone + Default> From<&[T]> for List<T> {
    fn from(slice: &[T]) -> Self {
        let mut list = List::new();
        for item in slice {
            list.push(item.clone());
        }
        list
    }
}
/// Square-bracket indexing into a [List].
/// Accesses the node correspinding to [NodeIndex] `idx`.
impl<T: Default> Index<NodeIndex> for List<T> {
    type Output = Node<T>;
    fn index(&self, idx: NodeIndex) -> &Self::Output {
        &self.nodes[idx.0]
    }
}
impl<T: Default> IndexMut<NodeIndex> for List<T> {
    fn index_mut(&mut self, idx: NodeIndex) -> &mut Self::Output {
        &mut self.nodes[idx.0]
    }
}
