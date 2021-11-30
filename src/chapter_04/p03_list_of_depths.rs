//!
//! # List of Depths:
//!
//! Given a binary tree, design an algorithm which creates a linked list of all the nodes at each depth
//! (e.g., if you have a tree with depth 0, you'll have 0 linked lists).
//!
//! Hints: #107, #123, #735
//!

use crate::{
    binary_tree::{BinaryTree, NodePtr as BtreePtr},
    ptr_list::List,
};

/// Primary Implementation
///
/// In-order traverse tree nodes, keeping track of the depth at each,
/// and appending each node encountered to a linked-list indexed with its depth.
///
pub fn list_of_depths(tree: &BinaryTree<isize>) -> Vec<List> {
    let mut results = Vec::new();
    helper(&tree.head, 0, &mut results);
    results
}

/// Recursive helper, doing all the real work
fn helper(ptr: &Option<BtreePtr<isize>>, level: usize, results: &mut Vec<List>) {
    // Unwrap the option around `ptr`, returning if it is `None`.
    let ptr = match ptr {
        None => return, // Base case: no node, no children
        Some(_) => ptr.as_ref().unwrap(),
    };

    // If it's our first time on this level, add an empty list to `results`.
    if results.len() <= level {
        results.push(List::default());
    }

    // In-order traverse nodes, adding to the linked-list at their `level`
    helper(&ptr.borrow().left, level + 1, results);
    results[level].add(ptr.borrow().data);
    helper(&ptr.borrow().right, level + 1, results);
}

#[test]
fn test_list_of_depths() {
    use crate::{binary_tree::Node as BtreeNode, ptr::Ptr};

    let node_h = Ptr::new(BtreeNode::new(7, None, None));
    let node_g = Ptr::new(BtreeNode::new(6, None, None));
    let node_f = Ptr::new(BtreeNode::new(5, None, None));
    let node_e = Ptr::new(BtreeNode::new(4, Some(node_g.clone()), None));
    let node_d = Ptr::new(BtreeNode::new(3, Some(node_h.clone()), None));
    let node_c = Ptr::new(BtreeNode::new(2, None, Some(node_f.clone())));
    let node_b = Ptr::new(BtreeNode::new(
        1,
        Some(node_d.clone()),
        Some(node_e.clone()),
    ));
    let node_a = Ptr::new(BtreeNode::new(
        0,
        Some(node_b.clone()),
        Some(node_c.clone()),
    ));
    let lists = list_of_depths(&BinaryTree {
        head: Some(node_a.clone()),
    });

    assert_eq!(lists[0].to_vec(), vec![0]);
    assert_eq!(lists[1].to_vec(), vec![1, 2]);
    assert_eq!(lists[2].to_vec(), vec![3, 4, 5]);
    assert_eq!(lists[3].to_vec(), vec![7, 6]);
}
