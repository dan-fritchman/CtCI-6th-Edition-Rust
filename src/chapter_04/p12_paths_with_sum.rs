//!
//! # Paths with Sum:
//!
//! You are given a binary tree in which each node contains an integer value (which might be positive or negative).
//! Design an algorithm to count the number of paths that sum to a given value.
//! The path does not need to start or end at the root or a leaf, but it must go downwards (traveling only from parent nodes to child nodes).
//!
//! Hints: #6, #74, #52, #68, #77, #87, #94, #703, #708, #115
//!

use crate::binary_tree::BinaryTree;
#[cfg(test)]
use crate::binary_tree::Error;

pub fn paths_with_sum(_tree: &BinaryTree<isize>, _sum: isize) -> usize {
    todo!()
}

#[ignore]
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

    assert_eq!(paths_with_sum(&t1, 8), usize::MAX);
    assert_eq!(paths_with_sum(&t1, 6), usize::MAX);

    Ok(())
}
