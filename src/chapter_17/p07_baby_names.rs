//!
//! # Baby Names:
//!
//! Each year, the government releases a list of the 10000 most common baby names and their frequencies (the number of babies with that name).
//! The only problem with this is that some names have multiple spellings.
//! For example, "John" and "Jon" are essentially the same name but would be listed separately in the list.
//! Given two lists, one of names/frequencies and the other of pairs of equivalent names,
//! write an algorithm to print a new list of the true frequency of each name.
//! Note that if John and Jon are synonyms, and Jon and Johnny are synonyms, then John and Johnny are synonyms.
//! (It is both transitive and symmetric.)
//! In the final list, any name can be used as the "real" name.
//!
//! EXAMPLE
//! Input:
//! Names: John (15), Jon (12), Chris (13), Kris (4), Christopher (19)
//! Synonyms: (Jon, John), (John, Johnny), (Chris, Kris), (Chris, Christopher)
//! Output: John (27), Kris (36)
//!
//! Hints:#478, #493,#512,#53~#586,#605,#655, #675,#704
//!

use std::collections::{HashMap, HashSet};

/// Primary Implementation
///
/// Create a [SynonymGraph] from synonym-pairs,
/// then search it depth-first for the sum over each connected region.
///
pub fn baby_names(counts: &[(&str, usize)], synonyms: &[(&str, &str)]) -> HashMap<String, usize> {
    // Create the [SynonymGraph]
    let graph = SynonymGraph::create(synonyms);

    // Re-work the `counts` into a [HashMap]
    let count_map: HashMap<&str, usize> = counts.iter().cloned().collect();

    // Create the [DfsSummer] helper, and return the counts it finds
    let mut summer = DfsSummer {
        graph: &graph,
        counts: count_map,
        seen: HashSet::new(),
    };
    summer.count()
}

/// Graph between synonymous names
/// Nodes are stored in a [HashMap], in which the keys equal their `name` field.
#[derive(Debug, Default)]
pub struct SynonymGraph {
    nodes: HashMap<String, Node>,
}
/// Synonym Graph Node
/// Name and a [HashSet] of (direct) neighbors
#[derive(Debug, Default)]
pub struct Node {
    _name: String,
    edges: HashSet<String>,
}
impl SynonymGraph {
    /// Create a [SynonymGraph] from synonym-pair list `synonyms`
    fn create(synonyms: &[(&str, &str)]) -> SynonymGraph {
        let mut graph = SynonymGraph::default();
        for pair in synonyms {
            // For each pair, add edges, and new [Node]s if necessary.
            graph.add(pair.0, pair.1);
            graph.add(pair.1, pair.0);
        }
        graph
    }
    /// Connect an edge from `name` to `conn`,
    /// creating a [Node] for `name` inline if necessary
    fn add(&mut self, name: &str, conn: &str) {
        self.nodes
            .entry(name.to_string())
            .or_insert_with(|| Node {
                _name: name.to_string(),
                edges: HashSet::new(),
            })
            .edges
            .insert(conn.to_string());
    }
}

/// Summation Helper
/// Depth-first traverses its [SynonymGraph] to find totals over connected regions.
#[derive(Debug)]
pub struct DfsSummer<'a> {
    graph: &'a SynonymGraph,
    counts: HashMap<&'a str, usize>,
    seen: HashSet<String>,
}
impl<'a> DfsSummer<'a> {
    /// Create and return a [HashMap] of counts of each name
    fn count(&mut self) -> HashMap<String, usize> {
        // Peel out a separate vector of names, to avoid some borrow-checker fights below.
        let names = self
            .counts
            .keys()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let mut rv: HashMap<String, usize> = HashMap::new();
        for name in names {
            if !self.graph.nodes.contains_key(&name) {
                // Not in the synonym-graph, add the value from `counts`
                rv.insert(name.clone(), *self.counts.get(name.as_str()).unwrap());
            } else if !self.seen.contains(&name) {
                // In the synonym-graph, and not previously processed.
                // Recursively add up its count and its neighbors.
                rv.insert(name.clone(), self.dfs_sum(&name));
            }
        }
        rv
    }
    /// Recursive summation helper
    /// Get the count for a region connected to node `name`.
    /// Returns zero if `name` has already been visited.
    fn dfs_sum(&mut self, name: &str) -> usize {
        if self.seen.contains(name) {
            return 0;
        }
        self.seen.insert(name.to_string());
        let mut count = *self.counts.get(name).unwrap();
        for neighbor in self.graph.nodes.get(name).unwrap().edges.iter() {
            count += self.dfs_sum(neighbor);
        }
        count
    }
}

#[test]
fn test_baby_names() {
    let counts = [
        ("john", 10),
        ("jon", 3),
        ("davis", 2),
        ("kari", 3),
        ("johny", 11),
        ("carlton", 8),
        ("carleton", 2),
        ("jonathan", 9),
        ("carrie", 5),
    ];

    let synonyms = [
        ("jonathan", "john"),
        ("jon", "johny"),
        ("johny", "john"),
        ("kari", "carrie"),
        ("carleton", "carlton"),
    ];

    // Run the code under test
    let result = baby_names(&counts, &synonyms);

    // Since key-selection is undeterministic, this takes some more work than `assert_eq(result)` to test.
    assert_eq!(result.keys().len(), 4);
    let jon = *result
        .get("jon")
        .or(result.get("john"))
        .or(result.get("johny"))
        .or(result.get("jonathan"))
        .unwrap();
    assert_eq!(jon, 33);
    let carlton = *result.get("carlton").or(result.get("carleton")).unwrap();
    assert_eq!(carlton, 10);
    let carrie = *result.get("carrie").or(result.get("kari")).unwrap();
    assert_eq!(carrie, 8);
    assert_eq!(*result.get("davis").unwrap(), 2);
}
