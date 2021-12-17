//!
//! # BST Sequences:
//!
//! A binary search tree was created by traversing through an array from left to right and inserting each element.
//! Given a binary search tree with distinct elements, print all possible arrays that could have led to this tree.
//!
//! EXAMPLE
//! Input:
//!   (1)
//! (2) (3)
//! Output:
//! {2, 1, 3}, {2, 3, 1}
//!
//! Hints: #39, #48, #66, #82
//!

use crate::binary_search_tree::{BinarySearchTree, NodeIndex};

/// Primary Implementation
///
/// Weave together combinations for each pair of sequences from each node's left and right sub-trees.
pub fn bst_sequences(tree: &BinarySearchTree) -> Vec<Vec<isize>> {
    helper(tree, &tree.head)
}

/// Recursive helper
fn helper(tree: &BinarySearchTree, ptr: &Option<NodeIndex>) -> Vec<Vec<isize>> {
    let idx = match ptr {
        None => return vec![vec![]], // Base case: no node, empty list of combinations
        Some(i) => *i,               // Otherwise unwrap the underlying node-index
    };

    // Get a set for each sub-tree
    let left = helper(tree, &tree[idx].left);
    let right = helper(tree, &tree[idx].right);

    // And weave together all combinations of each
    let mut results = Vec::new();
    for l in left.iter() {
        for r in right.iter() {
            let weaved = weave(l, r, &[]);
            results.extend(weaved);
        }
    }

    // Finally prepend the node's value to each list in `results`
    prepend(&[tree[idx].data], results)
}

/// Weave together all combinations of lists `a` and `b`, retaining the ordering among elements within `a` and `b`.
fn weave(a: &[isize], b: &[isize], prefix: &[isize]) -> Vec<Vec<isize>> {
    if a.is_empty() {
        return vec![[prefix, b].concat()]; // Base case: empty `a`, return single entry [[prefix] + [b]]
    }
    if b.is_empty() {
        return vec![[prefix, a].concat()]; // Base case: empty `b`, return single entry [[prefix] + [a]]
    }
    // Both `a` and `b` have elements. Do some real work.
    // For each leading sub-sequence in each of `a` and `b`, weave in the rest.
    let mut results = Vec::new();
    for aidx in 0..a.len() {
        let weaved = weave(&a[aidx + 1..a.len()], b, &a[0..aidx + 1]);
        results.extend(prepend(prefix, weaved));
    }
    for bidx in 0..b.len() {
        let weaved = weave(a, &b[bidx + 1..b.len()], &b[0..bidx + 1]);
        results.extend(prepend(prefix, weaved));
    }
    results
}

/// Prepend `prefix` to each element of `suffixes`
fn prepend(prefix: &[isize], suffixes: Vec<Vec<isize>>) -> Vec<Vec<isize>> {
    suffixes.iter().map(|s| [prefix, s].concat()).collect()
}

#[test]
fn test_bst_sequences() {
    let mut bst = BinarySearchTree::default();
    bst.insert(2);
    bst.insert(1);
    bst.insert(3);

    let sequences = bst_sequences(&bst);
    assert_eq!(sequences, &[[2, 1, 3], [2, 3, 1]]);

    let mut bst = BinarySearchTree::default();
    bst.insert(20);
    bst.insert(9);
    bst.insert(25);
    bst.insert(5);
    bst.insert(12);

    let sequences = bst_sequences(&bst);
    let correct = [
        [20, 12, 9, 5, 25],
        [20, 12, 9, 25, 5],
        [20, 12, 9, 5, 25],
        [20, 12, 25, 9, 5],
        [20, 12, 9, 5, 25],
        [20, 12, 9, 25, 5],
        [20, 12, 9, 5, 25],
        [20, 25, 12, 9, 5],
    ];
    assert_eq!(sequences, &correct);
}
