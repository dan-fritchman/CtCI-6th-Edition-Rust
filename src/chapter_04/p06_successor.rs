//!
//! # Successor:
//!
//! Write an algorithm to find the "next" node (i.e., in-order successor) of a given node in a binary search tree.
//! You may assume that each node has a link to its parent.
//!
//! Hints: #79, #91
//!

use std::ops::{Index, IndexMut};

///
/// # Another Binary Tree Node!
///
/// This time with a parent pointer, and using index-references.
///
#[derive(Debug)]
pub struct Node {
    pub data: isize,
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
    /// Insert a new [Node] with value `val`
    pub fn insert(&mut self, val: isize) {
        // Create and add the new node
        let newindex = self.add_node(val);

        let mut parent = match self.head {
            Some(p) => p.clone(),
            None => {
                // First node. Make it our `head` pointer.
                return self.head = Some(newindex);
            }
        };
        loop {
            if self[parent].data > val {
                // Look left
                match self[parent].left {
                    Some(idx) => {
                        if self[idx].data < val {
                            // Node goes here, as `parent`s left and with `idx` as its left sub-tree
                            self[newindex].parent = Some(parent);
                            self[parent].left = Some(newindex);
                            self[newindex].left = Some(idx);
                            self[idx].parent = Some(newindex);
                            break;
                        } else {
                            parent = idx;
                            continue;
                        }
                    }
                    None => {
                        self[newindex].parent = Some(parent);
                        self[parent].left = Some(newindex);
                        break;
                    }
                }
            } else {
                // Look right
                match self[parent].right {
                    Some(idx) => {
                        if self[idx].data > val {
                            // Node goes here, as `parent`s right and with `idx` as its right sub-tree
                            self[newindex].parent = Some(parent);
                            self[parent].right = Some(newindex);
                            self[newindex].right = Some(idx);
                            self[idx].parent = Some(newindex);
                            break;
                        } else {
                            parent = idx;
                            continue;
                        }
                    }
                    None => {
                        self[newindex].parent = Some(parent);
                        self[parent].right = Some(newindex);
                        break;
                    }
                }
            }
        }
    }
    /// Add a new [Node] with value `data`. Returns its index.
    fn add_node(&mut self, data: isize) -> NodeIndex {
        let index = NodeIndex(self.nodes.len());
        self.nodes.push(Node {
            data,
            index,
            parent: None,
            left: None,
            right: None,
        });
        index
    }
    /// In-order traversal, creating and returning a vector of data-elements
    pub fn inorder(&self) -> Vec<isize> {
        let mut v = Vec::new();
        self._inorder(&self.head, &mut v);
        v
    }
    /// Recursive helper for `inorder`
    fn _inorder(&self, opt: &Option<NodeIndex>, v: &mut Vec<isize>) {
        match opt {
            None => (),
            Some(idx) => {
                self._inorder(&self[*idx].left, v);
                v.push(self[*idx].data);
                self._inorder(&self[*idx].right, v);
            }
        }
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
}

/// Primary Implementation
///
/// First find the node (index) with value `val`.
/// From it, the successor comes from one of two places:
/// * If the node has right-children, it is the leftmost among them.
/// * If it does not, start traversing towards the head of the tree.
///   The successor is the first parent node found if coming *from* its left child,
///   if any exists. If not, the node is the tree's rightmost, and has no successor.
///  
pub fn successor(tree: &BinarySearchTree, val: isize) -> Option<isize> {
    // First get the node-index with value `val`, or fail.
    let mut idx = tree.find(val)?;

    // If that node has a right sub-tree, return its leftmost element
    if let Some(right) = tree[idx].right {
        let succ = leftmost(tree, right);
        return Some(tree[succ].data);
    }

    // No right sub-tree. The successor is the next parent which we hit coming from the left.
    while let Some(pidx) = tree[idx].parent {
        if tree[pidx].data > tree[idx].data {
            return Some(tree[pidx].data);
        }
        idx = pidx;
    }

    None // No successor found. This is the largest node in the tree.
}

/// Get the left-most child of node-index `idx`
fn leftmost(tree: &BinarySearchTree, mut idx: NodeIndex) -> NodeIndex {
    while let Some(left) = tree[idx].left {
        idx = left;
    }
    idx
}

#[test]
fn test_successor() {
    let mut bst = BinarySearchTree::default();
    bst.insert(20);
    bst.insert(9);
    bst.insert(25);
    bst.insert(5);
    bst.insert(12);
    bst.insert(11);
    bst.insert(14);

    // Quick check that things appear in order
    assert_eq!(bst.inorder(), &[5, 9, 11, 12, 14, 20, 25]);

    // And the real tests of the `successor` function
    let inputs = [5, 9, 11, 12, 14, 20, 25];
    for k in 0..inputs.len() - 1 {
        assert_eq!(successor(&bst, inputs[k]), Some(inputs[k + 1]));
    }
}
