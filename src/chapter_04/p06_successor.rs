//!
//! # Successor:
//!
//! Write an algorithm to find the "next" node (i.e., in-order successor) of a given node in a binary search tree.
//! You may assume that each node has a link to its parent.
//!
//! Hints: #79, #91
//!

use crate::binary_search_tree::{BinarySearchTree, NodeIndex};

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
