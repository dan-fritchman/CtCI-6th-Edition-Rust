//!
//! # Build Order:
//!
//! You are given a list of projects and a list of dependencies (which is a list of pairs of projects, where the second project is dependent on the first project).
//! All of a project's dependencies must be built before the project is.
//! Find a build order that will allow the projects to be built.
//! If there is no valid build order, return an error.
//!
//! EXAMPLE
//! Input:
//!   projects: a, b, c, d, e, f
//!   dependencies: (a, d), (f, b), (b, d), (f, a), (d, c)
//! Output:
//!   f, e, a, b, d, c
//!
//! Hints: #26, #47, #60, #85, # 125, # 733
//!

use std::collections::{HashMap, HashSet};

///
/// HashMap-Based Project Dependency Graph
///
#[derive(Debug, Default)]
struct Graph {
    projects: HashMap<char, Vec<char>>,
}
impl Graph {
    /// Create a [Graph] from the problem-stated lists of `projects` and `deps`.
    fn from(projects: &[char], deps: &[(char, char)]) -> Self {
        let mut this = Graph::default();
        // Add each project
        for p in projects {
            this.projects.insert(*p, Vec::new());
        }
        // And add each dependency-pair
        for (from, to) in deps {
            this.projects.get_mut(from).unwrap().push(*to);
        }
        this
    }
}

///
/// # Topological Graph Sorter
///
#[derive(Debug)]
struct TopoSorter<'g> {
    graph: &'g Graph,
    done: HashSet<char>,
    pending: HashSet<char>,
    order: Vec<char>,
}
impl<'g> TopoSorter<'g> {
    /// Create a new [TopoSorter] for [Graph] `graph`
    fn new(graph: &'g Graph) -> Self {
        Self {
            graph,
            done: HashSet::new(),
            pending: HashSet::new(),
            order: Vec::new(),
        }
    }
    /// Sort the graph elements. Returns a vector of project-chars if successful.
    /// Consumes the Sorter in the process.
    fn sort(mut self) -> Result<Vec<char>, CycleError> {
        // Visit each project
        for p in self.graph.projects.keys() {
            self.visit(*p)?;
        }
        // And return our ordered list
        Ok(self.order)
    }
    fn visit(&mut self, project: char) -> Result<(), CycleError> {
        if self.done.contains(&project) {
            return Ok(()); // Already visited, carry on
        }

        // If this node is already pending, we have a graph cycle.
        if self.pending.contains(&project) {
            return Err(CycleError); // Graph cycle!
        }
        self.pending.insert(project.clone());

        // First visit each of the project's dependencies
        for dep in self.graph.projects.get(&project).unwrap().iter() {
            self.visit(*dep)?;
        }

        // And handle the project itself, moving it from `pending` to `done` and into `order`.
        self.pending.remove(&project);
        self.done.insert(project.clone());
        self.order.push(project);
        Ok(())
    }
}

/// Primary Implementation
///
/// Create a [Graph], then create a [TopoSorter] to sort its nodes.
///
pub fn build_order(projects: &[char], deps: &[(char, char)]) -> Result<Vec<char>, CycleError> {
    let graph = Graph::from(projects, deps);
    TopoSorter::new(&graph).sort()
}

/// Error Type for Graph Cycles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CycleError;

#[test]
fn test_build_order() {
    let projects = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let dependencies = [
        ('d', 'g'),
        ('a', 'e'),
        ('b', 'e'),
        ('c', 'a'),
        ('f', 'a'),
        ('b', 'a'),
        ('f', 'c'),
        ('f', 'b'),
    ];

    // Run our sorting function
    let order = build_order(&projects, &dependencies).unwrap();

    // Since this order is not deterministic, it takes a more painful "search" for correctness.
    // Check each project does not depend on any project that comes after it.
    let mut depset = HashSet::new();
    for d in dependencies {
        depset.insert(d.clone());
    }
    for (idx, earlier) in order.iter().enumerate() {
        for later in order[idx..order.len()].iter() {
            assert!(!depset.contains(&(*earlier, *later)));
        }
    }

    // Test a failing, cyclical case
    let projects = ['a', 'b'];
    let dependencies = [('a', 'b'), ('b', 'a')];
    let order = build_order(&projects, &dependencies);
    assert_eq!(order, Err(CycleError));
}
