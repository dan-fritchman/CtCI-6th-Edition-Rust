//!
//! # Random Node:
//!
//! You are implementing a binary tree class from scratch which, in addition to insert, find, and delete,
//! has a method getRandomNode() which returns a random node from the tree.
//! All nodes should be equally likely to be chosen.
//! Design and implement an algorithm for getRandomNode, and explain how you would implement the rest of the methods.
//!
//! Hints: #42, #54, #62, #75, #89, #99, #7 72, #7 79
//!

// This models McDowell's "Option #7", probably the most-optimal solution.
// Each node keeps track of the size of its sub-tree.
// A random integer between 0 and `root.size` dictates which node
// is randomly selected.

use std::ops::{Index, IndexMut};

///
/// # (Yet) Another Binary Tree Node!
///
/// This time with a size of its sub-tree.
///
#[derive(Debug)]
pub struct Node {
    pub data: isize,
    pub size: usize,
    pub index: NodeIndex,
    pub left: Option<NodeIndex>,
    pub right: Option<NodeIndex>,
    pub parent: Option<NodeIndex>,
}
impl Node {
    /// Create a new [Node] with no children, and no parent
    pub fn orphan(data: isize, index: NodeIndex) -> Self {
        Self {
            data,
            size: 1,
            index,
            left: None,
            right: None,
            parent: None,
        }
    }
}

/// Index Reference into a Tree
#[derive(Debug, Clone, Copy)]
pub struct NodeIndex(usize);

/// Binary Tree
#[derive(Debug)]
pub struct BinarySearchTree {
    pub nodes: Vec<Node>,
    pub head: Option<NodeIndex>,
}
impl Default for BinarySearchTree {
    fn default() -> Self {
        Self {
            nodes: Vec::new(),
            head: None,
        }
    }
}
impl Index<NodeIndex> for BinarySearchTree {
    type Output = Node;
    fn index(&self, index: NodeIndex) -> &Node {
        &self.nodes[index.0]
    }
}
impl IndexMut<NodeIndex> for BinarySearchTree {
    fn index_mut(&mut self, index: NodeIndex) -> &mut Node {
        &mut self.nodes[index.0]
    }
}

impl BinarySearchTree {
    /// Add a new [Node] with value `data`. Returns its index.
    fn add_node(&mut self, data: isize) -> NodeIndex {
        let index = NodeIndex(self.nodes.len());
        self.nodes.push(Node {
            data,
            index,
            size: 1,
            parent: None,
            left: None,
            right: None,
        });
        index
    }
    /// Insert a new [Node] with value `val`. Returns its [NodeIndex].
    pub fn insert(&mut self, val: isize) -> Result<NodeIndex, Error> {
        // Create and add the new node
        let newindex = self.add_node(val);

        let mut parent = match self.head {
            Some(p) => p,
            None => {
                // First node. Make it our `head` pointer.
                self.head = Some(newindex);
                return Ok(newindex);
            }
        };
        loop {
            self[parent].size += 1;
            if val < self[parent].data {
                // Look left
                match self[parent].left {
                    Some(idx) => parent = idx, // Node will be somewhere in `idx`'s sub-tree
                    None => {
                        // No child here, insert new node in place
                        self[newindex].parent = Some(parent);
                        self[parent].left = Some(newindex);
                        break;
                    }
                }
            } else {
                // Look right
                match self[parent].right {
                    Some(idx) => parent = idx, // Node will be somewhere in `idx`'s sub-tree
                    None => {
                        // No child here, insert new node in place
                        self[newindex].parent = Some(parent);
                        self[parent].right = Some(newindex);
                        break;
                    }
                }
            }
        }
        return Ok(newindex);
    }
    /// Find the node with value `val`, if present
    pub fn find(&self, val: isize) -> Option<NodeIndex> {
        let mut node = self.head;
        while let Some(idx) = node {
            if self[idx].data == val {
                return Some(idx);
            }
            node = if self[idx].data > val {
                self[idx].left
            } else {
                self[idx].right
            };
        }
        None // Not found
    }
    /// Get a uniformly-distributed random [NodeIndex].
    /// Returns `None` if the tree is empty.
    pub fn get_random(&self) -> Option<NodeIndex> {
        // Check for emptiness, and get our root-node.
        let head = match self.head {
            None => return None,
            Some(h) => h,
        };

        // Grab a random, uniformly-distributed integer,
        // within the bounds of our `head`'s size.
        let random_num = {
            use rand::distributions::{Distribution, Uniform};

            let mut rng = rand::thread_rng();
            let dist = Uniform::from(0..self[head].size);
            dist.sample(&mut rng)
        };

        // And kick off the recursive search for the `random_num`-th
        // node in an in-order traversal.
        Some(self.random_helper(head, random_num))
    }
    /// Recursive helper for `get_random`.
    /// Returns the `rand`-th in-order node of the sub-tree headed by `node`.
    /// Panics if the sub-tree has less than `rand` nodes.
    fn random_helper(&self, node: NodeIndex, rand: usize) -> NodeIndex {
        if rand == 0 {
            return node; // Base case: `node` is it.
        }
        // If the node has a left-chld with enough children, recurse into it
        if let Some(lf) = self[node].left {
            if self[lf].size >= rand {
                return self.random_helper(lf, rand - 1);
            }
        }
        // Otherwise peel out the left-sub-tree size, and recurse into the right
        let left_size = match self[node].left {
            Some(l) => self[l].size,
            None => 0,
        };
        self.random_helper(self[node].right.unwrap(), rand - left_size - 1)
    }
}

/// Local Error Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Error;

/// Solution Implementation
/// Returns a random [NodeIndex] from `tree`.
/// Panics if `tree` is empty.
pub fn get_random_node(tree: &BinarySearchTree) -> NodeIndex {
    tree.get_random().unwrap()
}

#[test]
fn test_get_random_node() -> Result<(), Error> {
    use std::collections::HashMap;

    let mut bst = BinarySearchTree::default();
    let n20 = bst.insert(20)?;
    assert_eq!(bst[n20].size, 1);

    let n09 = bst.insert(9)?;
    assert_eq!(bst[n20].size, 2);
    assert_eq!(bst[n09].size, 1);

    let n25 = bst.insert(25)?;
    assert_eq!(bst[n20].size, 3);
    assert_eq!(bst[n09].size, 1);
    assert_eq!(bst[n25].size, 1);

    let n05 = bst.insert(5)?;
    assert_eq!(bst[n20].size, 4);
    assert_eq!(bst[n09].size, 2);
    assert_eq!(bst[n25].size, 1);
    assert_eq!(bst[n05].size, 1);

    let n12 = bst.insert(12)?;
    assert_eq!(bst[n20].size, 5);
    assert_eq!(bst[n09].size, 3);
    assert_eq!(bst[n25].size, 1);
    assert_eq!(bst[n05].size, 1);
    assert_eq!(bst[n12].size, 1);

    let n11 = bst.insert(11)?;
    assert_eq!(bst[n20].size, 6);
    assert_eq!(bst[n09].size, 4);
    assert_eq!(bst[n25].size, 1);
    assert_eq!(bst[n05].size, 1);
    assert_eq!(bst[n12].size, 2);
    assert_eq!(bst[n11].size, 1);

    let n14 = bst.insert(14)?;
    assert_eq!(bst[n20].size, 7);
    assert_eq!(bst[n09].size, 5);
    assert_eq!(bst[n25].size, 1);
    assert_eq!(bst[n05].size, 1);
    assert_eq!(bst[n12].size, 3);
    assert_eq!(bst[n11].size, 1);
    assert_eq!(bst[n14].size, 1);

    // We don't have great statistical tests for the uniformity of
    // random-node generation here, nor do we really want to spend
    // the run time checking them. But we can grab a random node
    // a sizable handful of times, and check that nothing goes terribly wrong.
    let mut counts = HashMap::new();
    for _ in 0..100 {
        let node = get_random_node(&bst);
        *counts.entry(node.0).or_insert(0) += 1;
    }
    Ok(())
}
