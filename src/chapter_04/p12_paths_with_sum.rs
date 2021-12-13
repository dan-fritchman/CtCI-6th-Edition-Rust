//!
//! # Paths with Sum:
//!
//! You are given a binary tree in which each node contains an integer value (which might be positive or negative).
//! Design an algorithm to count the number of paths that sum to a given value.
//! The path does not need to start or end at the root or a leaf, but it must go downwards (traveling only from parent nodes to child nodes).
//!
//! Hints: #6, #74, #52, #68, #77, #87, #94, #703, #708, #115
//!

#[cfg(test)]
use crate::binary_tree::Error;
use crate::binary_tree::{BinaryTree, NodePtr};

/// Primary Implementation
///
/// Track each path leading to each node, incrementing the paths-count where needed,
/// and and appending the node's value.
///
/// FIXME: this is closer to McDowell's "brute force" solution than the "optimized" hashmap-baseed on.
/// It's not clear that the hashmap solution actually does what the problem-statement asks,
/// particularly depending whether the paths which must add to `sum` must be
/// continuous or not.
/// There are no specific examples (with solutions) in McDowell,
/// so this point lacks some clarity.
/// There will certainly be cases in which this solution differs from the hashmap-one on this basis.
/// For example in a totally unbalanced tree of the form:
/// tree = 11 => 1 => 10
/// where "=>" can represent either "left child" or "right child".
/// Does `paths_with_sum(tree, 21)` equal one or zero?
/// In this case, zero. In the hashmap solution, one.
///
pub fn paths_with_sum(tree: &BinaryTree<isize>, sum: isize) -> usize {
    let paths = vec![];
    helper(&tree.head, &paths, sum)
}
/// Recursive helper
fn helper(node: &Option<NodePtr<isize>>, paths: &Vec<Vec<isize>>, sum: isize) -> usize {
    let node = match node {
        Some(n) => n,     // Unwrap the node-pointer
        None => return 0, // Base case: null node-pointer
    };
    let node_data = node.borrow().data;

    // Count up matches ending at this node
    let my_matches = paths
        .iter()
        .filter(|path| {
            let s: isize = path.iter().sum();
            s + node_data == sum
        })
        .count();

    // Append `node` to each entry of `paths`
    let mut child_paths: Vec<Vec<isize>> = paths
        .iter()
        .map(|s| {
            let mut s = s.to_vec();
            s.push(node_data);
            s
        })
        .collect();
    // And add a new path with `node` as the sole entry
    child_paths.push(vec![node_data]);

    // Total up this node's matches with those from each child
    my_matches
        + helper(&node.borrow().left, &child_paths, sum)
        + helper(&node.borrow().right, &child_paths, sum)
}
#[test]
fn test_paths_with_sum() -> Result<(), Error> {
    let mut t1 = BinaryTree::default();
    let n1 = t1.insert(10, None)?;
    let n2 = t1.insert(5, Some(n1.clone()))?;
    let n3 = t1.insert(-3, Some(n1))?;
    let n4 = t1.insert(3, Some(n2.clone()))?;
    let n5 = t1.insert(2, Some(n2))?;
    let _6 = t1.insert(3, Some(n4.clone()))?;
    let _7 = t1.insert(-2, Some(n4))?;
    let _8 = t1.insert(1, Some(n5))?;
    let n9 = t1.insert(11, Some(n3))?;
    let n10 = t1.insert(8, Some(n9))?;
    let _11 = t1.insert(-8, Some(n10))?;

    assert_eq!(paths_with_sum(&t1, 8), 4); // FIXME: verify these values offline
    assert_eq!(paths_with_sum(&t1, 6), 2); // FIXME: verify these values offline

    // Test case from the comments above
    let mut t1 = BinaryTree::default();
    let n1 = t1.insert(11, None)?;
    let n2 = t1.insert(1, Some(n1.clone()))?;
    let n3 = t1.insert(10, Some(n2.clone()))?;
    assert_eq!(paths_with_sum(&t1, 12), 1);
    assert_eq!(paths_with_sum(&t1, 21), 0);

    Ok(())
}
