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
    ops::Deref,
};

// Crates.io Imports
use typed_arena::Arena;

pub struct Error;
pub type Result<T> = std::result::Result<T, Error>;

/// # Internal Reference Type
///
/// Used to make all edge and node references.
/// Returned by [`Graph::add_node`] and [`Graph::add_edge`]
/// to enable further access and manipulation.
///
#[derive(Debug)]
pub struct Ref<'g, T>(&'g T);

impl<'g, T> Ref<'g, T> {
    /// Reference Constructor
    pub fn new(i: &'g T) -> Self {
        Self(i)
    }
    /// Get the address of this reference
    pub(crate) fn addr(&self) -> *const T {
        &*self.0 as *const T
    }
}
impl<'g, T> Deref for Ref<'g, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// Having a [Deref] implementation seems to screw with the auto-`derive`d implementations
// of a few key traits. Conveniently, they'ge all quite short.
impl<'g, T> Clone for Ref<'g, T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<'g, T> Copy for Ref<'g, T> {}

impl<'g, T> PartialEq for Ref<'g, T> {
    fn eq(&self, other: &Self) -> bool {
        self.addr() == other.addr()
    }
}
impl<'g, T> Eq for Ref<'g, T> {}
impl<'g, T> Hash for Ref<'g, T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.addr().hash(state)
    }
}

/// Type alias for references to nodes
type NodeRef<'g, N, E> = Ref<'g, Node<'g, N, E>>;
/// Type alias for references to edges
type EdgeRef<'g, N, E> = Ref<'g, Edge<'g, N, E>>;

#[derive(Clone)]
pub struct EdgeList<'g, N, E>(RefCell<Vec<EdgeRef<'g, N, E>>>);

/// Graph Node
/// Data type is `char`, a short "name" for each node.
pub struct Node<'g, N, E> {
    data: N,
    outgoing: EdgeList<'g, N, E>,
    incoming: EdgeList<'g, N, E>,
}
impl<'g, N, E> Node<'g, N, E> {
    pub fn new(data: N) -> Self {
        Self {
            data,
            outgoing: EdgeList(RefCell::new(Vec::new())),
            incoming: EdgeList(RefCell::new(Vec::new())),
        }
    }
}

/// Directed Edge
pub struct Edge<'g, N, E> {
    pub data: E,
    pub src: NodeRef<'g, N, E>,
    pub dst: NodeRef<'g, N, E>,
}
impl<'g, N, E> Edge<'g, N, E> {
    fn new(data: E, src: NodeRef<'g, N, E>, dst: NodeRef<'g, N, E>) -> Self {
        Self { data, src, dst }
    }
}

/// # Arena Wrapper
/// A light wrapper around [`typed_arena::Arena`] which:
///
/// * Converts the output of `alloc` from mutable to immutable references
/// * Keeps a hash-set of all [`Ref`]s created by `alloc`
///
struct ArenaWrapper<'g, T> {
    arena: Arena<T>,
    refs: RefCell<HashSet<Ref<'g, T>>>,
}
impl<'g, T> ArenaWrapper<'g, T> {
    fn new() -> Self {
        Self {
            arena: Arena::new(),
            refs: RefCell::new(HashSet::new()),
        }
    }
    fn alloc(&'g self, data: T) -> Ref<'g, T> {
        let inner = self.arena.alloc(data) as &T;
        let rf = Ref::new(inner);
        self.refs.borrow_mut().insert(rf.clone());
        rf
    }
}

/// # Directed Graph
///
/// Parameterized by the types of node-data `N` and edge-data `E`.
/// Both default to the unit type `()`, indicating no data.
///
pub struct Graph<'g, N = (), E = ()> {
    nodes: ArenaWrapper<'g, Node<'g, N, E>>,
    edges: ArenaWrapper<'g, Edge<'g, N, E>>,
}
impl<'g, N, E> Graph<'g, N, E> {
    /// Create a new, initially empty graph
    pub fn new() -> Self {
        Self {
            nodes: ArenaWrapper::new(),
            edges: ArenaWrapper::new(),
        }
    }
    /// Create and add a new [`Node`] from node-data `n`.
    /// Returns a [`Ref`] to the newly created node.
    pub fn create_node(&'g self, n: impl Into<N>) -> NodeRef<'g, N, E> {
        self.add_node(Node::new(n.into()))
    }
    /// Add a [`Node`] to the graph.
    /// Returns a [`Ref`] to the node.
    pub fn add_node(&'g self, node: Node<'g, N, E>) -> NodeRef<'g, N, E> {
        self.nodes.alloc(node)
    }
    /// Create and add a new edge from edge-data `e` and [`Node`] references `src and `dst`.
    /// Returns a [`Ref`] to the newly created edge.
    pub fn create_edge(
        &'g self,
        e: E,
        src: &NodeRef<'g, N, E>,
        dst: &NodeRef<'g, N, E>,
    ) -> EdgeRef<'g, N, E> {
        let edge = Edge::new(e, src.clone(), dst.clone());
        self.add_edge(edge)
    }
    /// Add an [`Edge`] to the graph.
    /// Returns a [`Ref`] to the edge.
    pub fn add_edge(&'g self, edge: Edge<'g, N, E>) -> EdgeRef<'g, N, E> {
        let edgeref = self.edges.alloc(edge);
        edgeref.src.outgoing.0.borrow_mut().push(edgeref.clone());
        edgeref.dst.incoming.0.borrow_mut().push(edgeref.clone());
        edgeref
    }
    // pub fn weight(&'g self, edge: &EdgeRef<'g, N, E>) -> usize {
    //     edge.data as usize // FIXME! 1 // FIXME! real edge weights for other edge-data types
    // }
}
// impl<'g, N, E: EdgeWeight> Graph<'g, N, E> {
//     pub fn weight(&'g self, edge: &EdgeRef<'g, N, E>) -> usize {
//         edge.weight()
//     }
// }

pub trait EdgeWeight {
    type Edge;
    fn weight(&self, edge: &Self::Edge) -> usize;
}
impl<'g, N> EdgeWeight for Graph<'g, N, usize> {
    type Edge = EdgeRef<'g, N, usize>;
    fn weight(&self, edge: &Self::Edge) -> usize {
        edge.data
    }
}
#[test]
fn test_dijkstra() {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct CharHolder(char);
    impl From<char> for CharHolder {
        fn from(c: char) -> Self {
            Self(c)
        }
    }
    let graph = Graph::<CharHolder, usize>::new();

    let a = graph.create_node('a');
    let b = graph.create_node('b');
    let c = graph.create_node('c');
    let d = graph.create_node('d');
    let e = graph.create_node('e');
    let f = graph.create_node('f');
    let g = graph.create_node('g');
    let h = graph.create_node('h');
    let i = graph.create_node('i');

    let _e = graph.create_edge(5, &a, &b);
    let e1 = graph.create_edge(3, &a, &c);
    let _e = graph.create_edge(2, &a, &e);
    let _e = graph.create_edge(2, &b, &d);
    let _e = graph.create_edge(1, &c, &b);
    let e5 = graph.create_edge(1, &c, &d);
    let _e = graph.create_edge(1, &d, &a);
    let e7 = graph.create_edge(2, &d, &g);
    let _e = graph.create_edge(1, &d, &h);
    let _e = graph.create_edge(1, &e, &a);
    let _e = graph.create_edge(4, &e, &h);
    let _e = graph.create_edge(7, &e, &i);
    let _e = graph.create_edge(3, &f, &b);
    let _e = graph.create_edge(1, &f, &g);
    let _e = graph.create_edge(3, &g, &c);
    let e15 = graph.create_edge(2, &g, &i);
    let _e = graph.create_edge(2, &h, &c);
    let _e = graph.create_edge(2, &h, &f);
    let _e = graph.create_edge(2, &h, &g);

    dbg!(&graph);
    let res = dijkstra(&graph, &a, &i);
    dbg!(&res);
    assert_eq!(
        res,
        Some(PathResult {
            cost: 8,
            src: a.clone(),
            dst: i.clone(),
            steps: vec![
                // FIXME: all these `some_edge`s are the wrong edges
                PathStep::new(a, e1),
                PathStep::new(c, e5),
                PathStep::new(d, e7),
                PathStep::new(g, e15),
            ]
        })
    )
}

/// Node + Score Combination, used in the priority heap
/// Inverts comparison order to make the std-lib max-heap serve as a min-heap.
struct NodeScore<'g, N> {
    node: Ref<'g, N>,
    score: usize,
}
impl<'g, N> NodeScore<'g, N> {
    pub fn new(node: Ref<'g, N>, score: usize) -> Self {
        Self { node, score }
    }
}
impl<'g, N> PartialEq for NodeScore<'g, N> {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl<'g, N> Eq for NodeScore<'g, N> {}
impl<'g, N> PartialOrd for NodeScore<'g, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'g, N> Ord for NodeScore<'g, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Priority reversal happens here, with the order of `self` and `other`
        other.score.cmp(&self.score)
    }
}

/// Dijkstra-Based Shortest-Path Solver,
/// from `src` to `dst` in [Graph] `graph`.
pub fn dijkstra<'g, G, N, E>(
    graph: &'g G,
    src: &NodeRef<'g, N, E>,
    dst: &NodeRef<'g, N, E>,
) -> Option<PathResult<'g, N, E>>
where
    N: Clone,
    E: Clone,
    G: EdgeWeight<Edge = EdgeRef<'g, N, E>>,
{
    // if !graph.nodes.refs.borrow().contains(src) || !graph.nodes.refs.borrow().contains(dst) {
    //     return None; // Check that the two nodes are in `graph`, or fail.
    // }

    // Initialize the remaining-queue and path-weight-map
    let mut q: BinaryHeap<NodeScore<'g, Node<'g, N, E>>> = BinaryHeap::new();
    q.push(NodeScore::new(src.clone(), 0));
    let mut weights: HashMap<NodeRef<'g, N, E>, usize> = HashMap::new();
    weights.insert(src.clone(), 0);

    // And initialize the previous-nodes mapping
    let mut previous: HashMap<NodeRef<'g, N, E>, PathStep<'g, N, E>> = HashMap::new();

    // The primary search loop
    while let Some(NodeScore { node, score }) = q.pop() {
        // For each outgoing edge, if we have a better path, update `weights` and `previous` pointers
        for edge in node.outgoing.0.borrow().iter() {
            let dst_weight = *weights.get(&edge.dst).unwrap_or(&usize::MAX);
            let new_weight = score.saturating_add(graph.weight(edge));
            if new_weight < dst_weight {
                // Update the scores and pointers for the destination
                let step = PathStep::new(node.clone(), edge.clone());
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
    let mut step: PathStep<'g, N, E> = previous.get(dst).cloned().unwrap();
    loop {
        steps.push(step.clone()); // Add the step to our path
        if step.node == *src {
            break; // Reached the source - done
        }
        step = match previous.get(&step.node) {
            Some(s) => s.clone(), // Get the previous step
            None => return None,  // No path found
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
pub struct PathStep<'g, N, E> {
    node: NodeRef<'g, N, E>,
    edge: EdgeRef<'g, N, E>,
}
impl<'g, N, E> PathStep<'g, N, E> {
    fn new(node: NodeRef<'g, N, E>, edge: EdgeRef<'g, N, E>) -> Self {
        Self { node, edge }
    }
}

/// Result from a successful shortest-path search
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PathResult<'g, N, E> {
    cost: usize,
    steps: Vec<PathStep<'g, N, E>>,
    src: NodeRef<'g, N, E>,
    dst: NodeRef<'g, N, E>,
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
/// ```text
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

    impl<'g, N: Debug, E: Debug> Debug for NodeRef<'g, N, E> {
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
    impl<'g, N: Debug, E: Debug> Debug for NodeSummary<'g, N, E> {
        /// Print the "summary" content of a [`Node`], including its address and data but not its edges.
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Node")
                .field("addr", &self.0.addr())
                .field("data", &self.0.data)
                .finish()
        }
    }
    impl<'g, N: Debug, E: Debug> Debug for EdgeRef<'g, N, E> {
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
    impl<'g, N: Debug, E: Debug> Debug for EdgeSummary<'g, N, E> {
        /// Print the "summary" content of an [`Edge`], including its address and data but not its connected nodes.
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Edge")
                .field("addr", &self.0.addr())
                .field("data", &self.0.data)
                .finish()
        }
    }
    impl<'g, N: Debug, E: Debug> Debug for EdgeList<'g, N, E> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_list().entries(self.0.borrow().iter()).finish()
        }
    }
    impl<'g, N: Debug, E: Debug> Debug for PathStep<'g, N, E> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_struct("Step")
                .field("node", &self.node)
                .field("edge", &self.edge)
                .finish()
        }
    }
    impl<'g, N: Debug, E: Debug> Debug for PathResult<'g, N, E> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            f.debug_list().entries(self.steps.iter()).finish()
        }
    }
    impl<'g, N: Debug, E: Debug> Debug for Graph<'g, N, E> {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            let nodes: Vec<_> = self.nodes.refs.borrow().iter().map(|n| n.clone()).collect();
            let edges: Vec<_> = self.edges.refs.borrow().iter().map(|n| n.clone()).collect();
            f.debug_struct("Graph")
                .field("nodes", &nodes)
                .field("edges", &edges)
                .finish()
        }
    }

    /// Private type for "summarizing" the content of a node,
    /// solely for [`Debug`] printing purposes.
    struct NodeSummary<'g, N, E>(NodeRef<'g, N, E>);

    /// Private type for "summarizing" the content of an edge,
    /// solely for [`Debug`] printing purposes.
    struct EdgeSummary<'g, N, E>(EdgeRef<'g, N, E>);

    /// "Summarize" a list of edges ([`EdgeList`]) as a vector of [`EdgeSummary`]s.
    fn summarize<'g, N, E>(elist: &EdgeList<'g, N, E>) -> Vec<EdgeSummary<'g, N, E>> {
        elist
            .0
            .borrow()
            .iter()
            .map(|e| EdgeSummary(e.clone()))
            .collect()
    }
}
