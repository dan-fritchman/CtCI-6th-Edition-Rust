//!
//! # Chapter 4 Utilities and Definitions
//!
//! Defines [Graph] and related types.
//!
//! Unlike the linked-lists chapter, these *do not* use the Rustic slot-map-style implementation.
//! Instead they use [Rc] and friends, much as outlined in the Rust "too many linked lists" book.
//!
//! Nodes are stored inside a newtype-wrapper of `Rc<RefCell>`,
//! which also adds by-address implementations of [Eq] and [Hash].
//! The former aids in comparing nodes by identify rather than value,
//! and the latter aids in using hash-based structures to store sets of nodes.
//!

use std::collections::HashSet;

// Grab the shared [Ptr] type, originally defined here and then moved to a dedicated module.
pub use crate::ptr::Ptr;

/// (Un-Weighted) Edge
#[derive(Debug)]
pub struct Edge<T> {
    pub src: NodePtr<T>,
    pub dst: NodePtr<T>,
}
impl<T> Clone for Edge<T> {
    fn clone(&self) -> Self {
        Self {
            src: NodePtr::clone(&self.src),
            dst: NodePtr::clone(&self.dst),
        }
    }
}
impl<T> Edge<T> {
    fn new(src: NodePtr<T>, dst: NodePtr<T>) -> Self {
        Self { src, dst }
    }
}

/// Directed-Graph Node
#[derive(Debug, Clone)]
pub struct Node<T> {
    pub data: T,
    pub outgoing: Vec<Edge<T>>,
    pub incoming: Vec<Edge<T>>,
}
impl<T> Node<T> {
    pub fn new(data: T) -> Node<T> {
        Self {
            data,
            incoming: Vec::new(),
            outgoing: Vec::new(),
        }
    }
}

/// Alias for a [Ptr] to a [Node]
pub type NodePtr<T> = Ptr<Node<T>>;

/// Directed Graph
#[derive(Debug, Clone)]
pub struct Graph<T> {
    nodes: HashSet<NodePtr<T>>,
}
impl<T> Default for Graph<T> {
    fn default() -> Self {
        Self {
            nodes: HashSet::new(),
        }
    }
}
impl<T> Graph<T> {
    /// Create an initially empty [Graph]. Also available via [Default].
    pub fn new() -> Self {
        Self::default()
    }
    /// Insert a [Node] with data-value `data` into the graph, returning a [NodePtr] to it.
    pub fn add(&mut self, data: T) -> NodePtr<T> {
        let ptr = NodePtr::new(Node::new(data));
        self.nodes.insert(ptr.clone());
        ptr
    }
    /// Add an [Edge] from `src` to `dst`
    pub fn connect(&mut self, src: &NodePtr<T>, dst: &NodePtr<T>) {
        // Create the new [Edge]
        let edge = Edge::new(src.clone(), dst.clone());
        // And append it to the source and destination [Node]s.
        // (Now the [Rc]/[RefCell] fun begins.)
        src.borrow_mut().outgoing.push(edge.clone());
        dst.borrow_mut().incoming.push(edge);
    }
    pub fn contains(&self, nodeptr: &NodePtr<T>) -> bool {
        self.nodes.contains(nodeptr)
    }
}
