//!
//! # Random Node:
//!
//! You are implementing a binary tree class from scratch which, in addition to insert, find, and delete, has a method getRandomNode()
//! which returns a random node from the tree.
//! All nodes should be equally likely to be chosen.
//! Design and implement an algorithm for getRandomNode, and explain how you would implement the rest of the methods.
//!
//! Hints: #42, #54, #62, #75, #89, #99, #7 72, #7 79
//!

#[cfg(test)]
use crate::binary_tree::Error;
use crate::binary_tree::{BinaryTree, NodePtr};

pub fn get_random_node<T>(tree: &BinaryTree<T>) -> NodePtr<T> {
    todo!()
}

#[ignore]
#[test]
fn test_get_random_node() -> Result<(), Error> {
    use std::collections::HashMap;

    let mut bst = BinaryTree::default();
    let n = bst.insert(20, None)?;
    let n = bst.insert(9, Some(n))?;
    let n = bst.insert(25, Some(n))?;
    let n = bst.insert(5, Some(n))?;
    let n = bst.insert(12, Some(n))?;
    let n = bst.insert(11, Some(n))?;
    let n = bst.insert(14, Some(n))?;
    // let n = bst.delete(12); // FIXME!

    let mut counts = HashMap::new();
    for _ in 0..100 {
        let node = get_random_node(&bst);
        let data = node.borrow().data;
        *counts.entry(data).or_insert(0) += 1;
    }
    Ok(())
}
