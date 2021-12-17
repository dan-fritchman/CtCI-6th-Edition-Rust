//! 
//! # Binary Search Tree 
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
            Some(p) => p,
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
