//!
//! # Weighted Directed Graphs, and Dijkstra-Based Shortest Paths
//!

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};
// Local Imports
pub use super::ptr::Ptr;

/// Graph Node
/// Data type is `char`, a short "name" for each node.
#[derive(Debug, Default)]
pub struct Node {
    data: char,
    outgoing: Vec<Edge>,
    incoming: Vec<Edge>,
}
impl Node {
    pub fn new(data: char) -> Self {
        Self {
            data,
            ..Default::default()
        }
    }
}

/// Type alias for a [Ptr] to a [Node]
pub type NodePtr = Ptr<Node>;

/// Weighted Edge
#[derive(Debug, Clone)]
pub struct Edge {
    pub weight: usize,
    pub src: NodePtr,
    pub dst: NodePtr,
}
impl Edge {
    fn new(weight: usize, src: NodePtr, dst: NodePtr) -> Self {
        Self { weight, src, dst }
    }
}

/// Directed Graph
#[derive(Debug, Clone, Default)]
pub struct Graph {
    nodes: HashSet<NodePtr>,
}
impl Graph {
    /// Insert a [Node] with data-value `data` into the graph, returning a [NodePtr] to it.
    pub fn add(&mut self, data: char) -> NodePtr {
        let ptr = NodePtr::new(Node::new(data));
        self.nodes.insert(ptr.clone());
        ptr
    }
    /// Add an [Edge] from `src` to `dst`
    pub fn connect(&mut self, weight: usize, src: &NodePtr, dst: &NodePtr) {
        // Create the new [Edge]
        let edge = Edge::new(weight, src.clone(), dst.clone());
        // And append it to the source and destination [Node]s.
        // (Now the [Rc]/[RefCell] fun begins.)
        src.borrow_mut().outgoing.push(edge.clone());
        dst.borrow_mut().incoming.push(edge);
    }
}

#[test]
fn test_dijkstra() {
    let mut graph = Graph::default();

    let a = graph.add('a');
    let b = graph.add('b');
    let c = graph.add('c');
    let d = graph.add('d');
    let e = graph.add('e');
    let f = graph.add('f');
    let g = graph.add('g');
    let h = graph.add('h');
    let i = graph.add('i');

    graph.connect(5, &a, &b);
    graph.connect(3, &a, &c);
    graph.connect(2, &a, &e);
    graph.connect(2, &b, &d);
    graph.connect(1, &c, &b);
    graph.connect(1, &c, &d);
    graph.connect(1, &d, &a);
    graph.connect(2, &d, &g);
    graph.connect(1, &d, &h);
    graph.connect(1, &e, &a);
    graph.connect(4, &e, &h);
    graph.connect(7, &e, &i);
    graph.connect(3, &f, &b);
    graph.connect(1, &f, &g);
    graph.connect(3, &g, &c);
    graph.connect(2, &g, &i);
    graph.connect(2, &h, &c);
    graph.connect(2, &h, &f);
    graph.connect(2, &h, &g);

    let res = solve(&graph, &a, &i);
    assert_eq!(
        res,
        Some(PathResult {
            cost: 8,
            path: vec!['a', 'c', 'd', 'g', 'i']
        })
    )
}

/// Node + Score Combination, used in the priority heap
/// Inverts comparison order to make the std-lib max-heap serve as a min-heap.
pub struct NodeScore {
    node: NodePtr,
    score: usize,
}
impl NodeScore {
    pub fn new(node: NodePtr, score: usize) -> Self {
        Self { node, score }
    }
}
impl PartialEq for NodeScore {
    fn eq(&self, other: &NodeScore) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for NodeScore {}
impl PartialOrd for NodeScore {
    fn partial_cmp(&self, other: &NodeScore) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for NodeScore {
    fn cmp(&self, other: &NodeScore) -> Ordering {
        // Priority reversal happens here, with the order of `self` and `other`
        other.score.cmp(&self.score)
    }
}

/// Dijkstra-Based Shortest-Path Solver,
/// from `src` to `dest` in [Graph] `graph`.
pub fn solve(graph: &Graph, src: &NodePtr, dest: &NodePtr) -> Option<PathResult> {
    if !graph.nodes.contains(src) || !graph.nodes.contains(dest) {
        return None; // Check that the two nodes are in `graph`, or fail.
    }

    // Initialize the remaining-queue and path-weight-map
    let mut q: BinaryHeap<NodeScore> = BinaryHeap::new();
    q.push(NodeScore::new(src.clone(), 0));
    let mut weights: HashMap<NodePtr, usize> = HashMap::new();
    weights.insert(src.clone(), 0);

    // And initialize the previous-nodes mapping
    let mut previous: HashMap<NodePtr, NodePtr> = HashMap::new();

    // The primary search loop
    while let Some(NodeScore { node, score }) = q.pop() {
        // For each outgoing edge, if we have a better path, update `weights` and `previous` pointers
        for edge in node.borrow().outgoing.iter() {
            let dest_weight = *weights.get(&edge.dst).unwrap_or(&usize::MAX);
            let new_weight = score + edge.weight;
            if new_weight < dest_weight {
                // Update the scores and pointers for the destination
                previous.insert(edge.dst.clone(), node.clone());
                weights.insert(edge.dst.clone(), new_weight);
                q.push(NodeScore::new(edge.dst.clone(), new_weight));
            }
        }
    }

    // Now unwind the path from `dest` back to `src`
    let mut path = Vec::new();
    let mut node: &NodePtr = dest;
    loop {
        path.push(node.borrow().data); // Add the node-name to our path
        if node == src {
            break; // Reached the source - done
        }
        node = match previous.get(node) {
            Some(n) => n,        // Get the previous node
            None => return None, // No path found
        };
    }
    // Reverse the traversed `path`, ordering from `src` to `dest`
    path.reverse();

    // Grab the cost of the destination node, or return None if not found
    let cost = match weights.get(dest) {
        Some(c) if *c != usize::MAX => *c,
        _ => return None,
    };

    // Success - return a [PathResult]
    Some(PathResult { cost, path })
}

/// Result from a successful shortest-path search
#[derive(Debug, Default, PartialEq, Eq)]
pub struct PathResult {
    cost: usize,
    path: Vec<char>,
}
