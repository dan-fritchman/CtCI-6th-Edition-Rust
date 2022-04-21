//!
//! # Arena-Based Graphs
//!
//! For the hearty of heart
//!

// Std-Lib Imports
use std::{
    cell::RefCell,
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
};

// Crates.io Imports
use by_address::ByAddress;
use typed_arena::Arena;

pub struct Error;
pub type Result<T> = std::result::Result<T, Error>;

/// # Internal Reference Type
///
/// Used to make all edge-node references.
/// Returned by [`Graph::add_node`] and [`Graph::add_edge`]
/// to enable further access and manipulation.
///
pub struct Ref<'g, T>(ByAddress<&'g T>);

impl<'r, T> Ref<'r, T> {
    /// Pointer Constructor
    pub fn new(i: &'r T) -> Self {
        Self(ByAddress(i))
    }
    /// Get the address of this reference
    pub(crate) fn addr(&self) -> *const T {
        &*self.0 .0
    }
}
impl<'r, T> Deref for Ref<'r, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0 .0
    }
}
// Having a [Deref] implementation seems to screw with the auto-`derive`d implementations
// of a few key traits. Conveniently, they're all quite short.
impl<'r, T> Clone for Ref<'r, T> {
    fn clone(&self) -> Self {
        Self(ByAddress::clone(&self.0))
    }
}
impl<'r, T> Copy for Ref<'r, T> {}

impl<'r, T> PartialEq for Ref<'r, T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl<'r, T> Eq for Ref<'r, T> {}
impl<'r, T> Hash for Ref<'r, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

/// Type alias for references to nodes
type NodeRef<'r> = Ref<'r, Node<'r>>;
/// Type alias for references to edges
type EdgeRef<'r> = Ref<'r, Edge<'r>>;

#[derive(Debug, Default, Clone)]
pub struct NodeData; // FIXME! Real content

#[derive(Clone)]
pub struct EdgeList<'r>(RefCell<Vec<EdgeRef<'r>>>);

/// Graph Node
/// Data type is `char`, a short "name" for each node.
#[derive(Clone)]
pub struct Node<'r> {
    data: NodeData,
    outgoing: EdgeList<'r>,
    incoming: EdgeList<'r>,
}
impl<'r> Node<'r> {
    pub fn new(data: NodeData) -> Self {
        Self {
            data,
            outgoing: EdgeList(RefCell::new(Vec::new())),
            incoming: EdgeList(RefCell::new(Vec::new())),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct EdgeData; // FIXME! Real content

/// Directed Edge
#[derive(Debug, Clone)]
pub struct Edge<'r> {
    pub data: EdgeData,
    pub src: NodeRef<'r>,
    pub dst: NodeRef<'r>,
}
impl<'r> Edge<'r> {
    fn new(data: EdgeData, src: NodeRef<'r>, dst: NodeRef<'r>) -> Self {
        Self { data, src, dst }
    }
}

struct ArenaWrapper<T>(Arena<T>);
impl<'g, T> ArenaWrapper<T> {
    fn new() -> Self {
        Self(Arena::new())
    }
    fn alloc(&'g self, data: T) -> Ref<'g, T> {
        let inner = self.0.alloc(data) as &T;
        Ref::new(inner)
    }
}

/// Graph
pub struct Graph<'r> {
    nodes: ArenaWrapper<Node<'r>>,
    noderefs: RefCell<HashSet<NodeRef<'r>>>,
    edges: ArenaWrapper<Edge<'r>>,
    edgerefs: RefCell<HashSet<EdgeRef<'r>>>,
}
impl<'r> Graph<'r> {
    pub fn new() -> Self {
        Self {
            nodes: ArenaWrapper::new(),
            noderefs: RefCell::new(HashSet::new()),
            edges: ArenaWrapper::new(),
            edgerefs: RefCell::new(HashSet::new()),
        }
    }
    pub fn add_node(&'r self, node: Node<'r>) -> NodeRef<'r> {
        let rf = self.nodes.alloc(node);
        self.noderefs.borrow_mut().insert(rf.clone());
        rf
    }
    pub fn add_edge(&'r self, edge: Edge<'r>) -> EdgeRef<'r> {
        let edgeref = self.edges.alloc(edge);
        edgeref.src.outgoing.0.borrow_mut().push(edgeref.clone());
        edgeref.dst.incoming.0.borrow_mut().push(edgeref.clone());
        self.edgerefs.borrow_mut().insert(edgeref.clone());
        edgeref
    }
    pub fn add_something(&'r self, _c: char) -> NodeRef<'r> {
        self.add_node(Node::new(NodeData))
    }
    pub fn connect(&'r self, _x: usize, src: &NodeRef<'r>, dst: &NodeRef<'r>) -> EdgeRef<'r> {
        let edge = Edge::new(EdgeData, src.clone(), dst.clone());
        self.add_edge(edge)
    }
    pub fn weight(&'r self, _edge: &EdgeRef<'r>) -> usize {
        1 // FIXME! Real weight
    }
}

#[test]
fn test_dijkstra() {
    let graph = Graph::new();

    let a = graph.add_something('a');
    let b = graph.add_something('b');
    let c = graph.add_something('c');
    let d = graph.add_something('d');
    let e = graph.add_something('e');
    let f = graph.add_something('f');
    let g = graph.add_something('g');
    let h = graph.add_something('h');
    let i = graph.add_something('i');

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

    dbg!(&graph);

    let res = dijkstra(&graph, &a, &i);
    dbg!(res);
    // assert_eq!(
    //     res,
    //     None
    //     // Some(PathResult {
    //     //     cost: 8,
    //     //     path: vec!['a', 'c', 'd', 'g', 'i']
    //     // })
    // )
}

/// Node + Score Combination, used in the priority heap
/// Inverts comparison order to make the std-lib max-heap serve as a min-heap.
struct NodeScore<'r, N> {
    node: Ref<'r, N>,
    score: usize,
}
impl<'r, N> NodeScore<'r, N> {
    pub fn new(node: Ref<'r, N>, score: usize) -> Self {
        Self { node, score }
    }
}
impl<'r, N> PartialEq for NodeScore<'r, N> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl<'r, N> Eq for NodeScore<'r, N> {}
impl<'r, N> PartialOrd for NodeScore<'r, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'r, N> Ord for NodeScore<'r, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Priority reversal happens here, with the order of `self` and `other`
        other.score.cmp(&self.score)
    }
}

/// Dijkstra-Based Shortest-Path Solver,
/// from `src` to `dst` in [Graph] `graph`.
pub fn dijkstra<'r>(
    graph: &'r Graph<'r>,
    src: &NodeRef<'r>,
    dst: &NodeRef<'r>,
) -> Option<PathResult<'r>> {
    if !graph.noderefs.borrow().contains(src) || !graph.noderefs.borrow().contains(dst) {
        return None; // Check that the two nodes are in `graph`, or fail.
    }

    // Initialize the remaining-queue and path-weight-map
    let mut q: BinaryHeap<NodeScore<'r, Node>> = BinaryHeap::new();
    q.push(NodeScore::new(src.clone(), 0));
    let mut weights: HashMap<Ref<'r, Node>, usize> = HashMap::new();
    weights.insert(src.clone(), 0);

    // And initialize the previous-nodes mapping
    let mut previous: HashMap<Ref<'r, Node>, PathStep<'r>> = HashMap::new();

    // The primary search loop
    while let Some(NodeScore { node, score }) = q.pop() {
        // For each outgoing edge, if we have a better path, update `weights` and `previous` pointers
        for edge in node.outgoing.0.borrow().iter() {
            let dst_weight = *weights.get(&edge.dst).unwrap_or(&usize::MAX);
            let new_weight = score.saturating_add(graph.weight(edge));
            if new_weight < dst_weight {
                // Update the scores and pointers for the destination
                let step = PathStep {
                    edge: edge.clone(),
                    node: node.clone(),
                };
                previous.insert(edge.dst.clone(), step);
                weights.insert(edge.dst.clone(), new_weight);
                q.push(NodeScore::new(edge.dst.clone(), new_weight));
            }
        }
    }

    // Grab the cost of the destination node, or return None if not found
    let cost = match weights.get(dst) {
        Some(c) if *c != usize::MAX => *c,
        _ => return None,
    };

    // Now unwind the path from `dst` back to `src`
    let mut steps = Vec::new();
    let mut step: &PathStep<'r> = previous.get(dst).unwrap();
    loop {
        steps.push(step.clone()); // Add the step to our path
        if step.node == *src {
            break; // Reached the source - done
        }
        step = match previous.get(&step.node) {
            Some(s) => s,        // Get the previous step
            None => return None, // No path found
        };
    }
    // Reverse the traversed `path`, ordering from `src` to `dst`
    steps.reverse();

    // Success - return a [PathResult]
    Some(PathResult {
        cost,
        dst: *dst,
        src: *src,
        steps,
    })
}

/// Step in a path, including a node and edge
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PathStep<'r> {
    node: NodeRef<'r>,
    edge: EdgeRef<'r>,
}

/// Result from a successful shortest-path search
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PathResult<'r> {
    cost: usize,
    steps: Vec<PathStep<'r>>,
    src: NodeRef<'r>,
    dst: NodeRef<'r>,
}

/// # Debug Printing Module
///
/// The default derived `Debug` implementations will generally hang
/// for cyclical structures such as [`Graph`], [`Node`], and [`Edge`],
/// as they infinitely try to print a cycle of dependencies.
///
/// Printing these instead requires choosing when to include their "full" content,
/// and when to "summarize" types, generally into their in-memory addresses
/// and associated data.
///
/// The format of the [`Debug`] output is then roughly:
///
/// ```
/// Node (
///     addr: 0xFFFFFFFF,
///     data: "1",
///     outgoing: [
///         Edge ( addr: 0xDEADBEEF, data: "A" ),
///         Edge ( addr: 0xABCD0123, data: "B" ),
///         Edge ( addr: 0x00000000, data: "C" ),
///     ],
///     incoming: [
///         // ...
///     ],
/// ),
/// // ...
/// Edge (
///     addr: 0xDEADBEEF,
///     src: Node ( addr: 0xFFFFFFFF, data: "1" ),
///     src: Node ( addr: 0x12345678, data: "2" ),
/// )
/// ```
///
/// Where:
/// * Nodes are listed first
///   * Each connected edge's print-out includes its address and data-value, but not all its node-connections.
/// * Edges are listed second
///   * Each connected node's print-out conversely includes its address and data-value, but not its connected edges.
///
mod debug {
    use super::*;
    use std::fmt::{Debug, Formatter, Result};

    impl<'r> Debug for NodeRef<'r> {
        /// Print the "full" content of a [`Node`], including its address, data, and edges.
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Node")
                .field("addr", &self.addr())
                .field("data", &self.data)
                .field("incoming", &summarize(&self.incoming))
                .field("outgoing", &summarize(&self.outgoing))
                .finish()
        }
    }
    impl<'r> Debug for NodeSummary<'r> {
        /// Print the "summary" content of a [`Node`], including its address and data but not its edges.
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Node")
                .field("addr", &self.0.addr())
                .field("data", &self.0.data)
                .finish()
        }
    }
    impl<'r> Debug for EdgeRef<'r> {
        /// Print the "full" content of an [`Edge`], including its address, data, and connected nodes.
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Edge")
                .field("addr", &self.addr())
                .field("data", &self.data)
                .field("src", &NodeSummary(self.src.clone()))
                .field("dst", &NodeSummary(self.dst.clone()))
                .finish()
        }
    }
    impl<'r> Debug for EdgeSummary<'r> {
        /// Print the "summary" content of an [`Edge`], including its address and data but not its connected nodes.
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Edge")
                .field("addr", &self.0.addr())
                .field("data", &self.0.data)
                .finish()
        }
    }
    impl<'r> Debug for EdgeList<'r> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_list().entries(self.0.borrow().iter()).finish()
        }
    }
    impl<'r> Debug for PathStep<'r> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Step")
                .field("node", &self.node)
                .field("edge", &self.edge)
                .finish()
        }
    }
    impl<'r> Debug for PathResult<'r> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_list().entries(self.steps.iter()).finish()
        }
    }
    impl<'r> Debug for Graph<'r> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let nodes: Vec<NodeRef<'_>> =
                self.noderefs.borrow().iter().map(|n| n.clone()).collect();
            let edges: Vec<EdgeRef<'_>> =
                self.edgerefs.borrow().iter().map(|n| n.clone()).collect();
            f.debug_struct("Graph")
                .field("nodes", &nodes)
                .field("edges", &edges)
                .finish()
        }
    }

    /// Private type for "summarizing" the content of a node,
    /// solely for [`Debug`] printing purposes.
    struct NodeSummary<'r>(NodeRef<'r>);
    /// Private type for "summarizing" the content of an edge,
    /// solely for [`Debug`] printing purposes.
    struct EdgeSummary<'r>(EdgeRef<'r>);

    /// "Summarize" a list of edges ([`EdgeList`]) as a vector of [`EdgeSummary`]s.
    fn summarize<'l>(elist: &EdgeList<'l>) -> Vec<EdgeSummary<'l>> {
        elist
            .0
            .borrow()
            .iter()
            .map(|e| EdgeSummary(e.clone()))
            .collect()
    }
}
