//!
//! # Route Between Nodes:
//!
//! Given a directed graph, design an algorithm to find out whether there is a route between two nodes.
//!
//! Hints: #127
//!

use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

// Local Imports
use super::utils::{Graph, NodePtr};

///
/// Primary Implementation Method
///
/// Breadth-first walk outward from `src`. If we hit `dst`, there is a path.
///
pub fn has_path<T>(graph: &Graph<T>, src: &NodePtr<T>, dst: &NodePtr<T>) -> Result<bool, ()> {
    // If the graph doesn't have these nodes, `false` doesn't quite cut it. Return an [Error].
    // (Note this is our only use of `graph`.)
    if !graph.contains(src) || !graph.contains(dst) {
        return Err(());
    }
    // Create the FIFO BFS queue, and previously-seen-nodes hash-set
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();

    // Add the source node to both the queue and seen-set
    seen.insert(src.clone());
    q.push_back(src.clone());

    // And look until the queue is empty
    while let Some(ptr) = q.pop_front() {
        if ptr == *dst {
            // Destination found - path found.
            return Ok(true);
        }
        // Push all the outgoing edges not yet seen onto the
        let node = ptr.borrow();
        for out in node.outgoing.iter() {
            if !seen.contains(&out.dst) {
                q.push_back(out.dst.clone());
                seen.insert(out.dst.clone());
            }
        }
    }
    // Getting here means we've exhausted every reachable node, without finding `dst`.
    // I.e. no path found.
    Ok(false)
}

#[test]
fn test_has_path() {
    // Create a [Graph] from an adjacency-list literal.
    let adj = vec![
        ("A", vec!["B", "C"]),
        ("B", vec!["D"]),
        ("C", vec!["D", "E"]),
        ("D", vec!["B", "C"]),
        ("E", vec!["C", "F"]),
        ("F", vec!["E", "O", "I", "G"]),
        ("G", vec!["F", "H"]),
        ("H", vec!["G"]),
        ("I", vec!["F", "J"]),
        ("O", vec!["F"]),
        ("J", vec!["K", "L", "I"]),
        ("K", vec!["J"]),
        ("L", vec!["J"]),
        ("P", vec!["Q", "R"]),
        ("Q", vec!["P", "R"]),
        ("R", vec!["P", "Q"]),
    ];

    let (graph, labels) = graph_from_adjacency(adj).unwrap();

    let test_cases = [
        ("A", "A", true),
        ("A", "B", true),
        ("A", "C", true),
        ("B", "B", true),
        ("A", "L", true),
        ("A", "B", true),
        ("H", "K", true),
        ("L", "D", true),
        ("P", "Q", true),
        ("Q", "P", true),
        ("Q", "G", false),
        ("R", "A", false),
        ("P", "B", false),
    ];
    for case in test_cases.iter() {
        let src = labels.get(case.0).unwrap();
        let dst = labels.get(case.1).unwrap();
        let has_path = has_path(&graph, src, dst).unwrap();
        assert_eq!(has_path, case.2);
    }
}

///
/// Create a graph from a data-valued adjacency list.
/// Returns both the graph and a mapping from labels to node-pointers.
///
pub fn graph_from_adjacency<T>(
    adj: Vec<(T, Vec<T>)>,
) -> Result<(Graph<T>, HashMap<T, NodePtr<T>>), ()>
where
    T: Clone + Hash + Eq,
{
    let mut graph = Graph::default();
    let mut labels = HashMap::new();

    // Create a temporary mapping from "labels" (data values) to nodes
    for entry in adj.iter() {
        let node = graph.add(entry.0.clone());
        labels.insert(entry.0.clone(), node);
    }
    // And for each entry in that mapping, add edges
    for entry in adj.iter() {
        let src = labels.get(&entry.0).ok_or(())?;
        for dst_label in entry.1.iter() {
            let dst = labels.get(dst_label).ok_or(())?;
            graph.connect(src, dst);
        }
    }
    Ok((graph, labels))
}
