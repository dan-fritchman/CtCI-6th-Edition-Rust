//!
//! # Minimal Tree:
//!
//! Given a sorted (increasing order) array with unique integer elements,
//! write an algorithm to create a binary search tree with minimal height.
//!
//! Hints: #19, #73, #176
//!

// Local Imports
use crate::binary_tree::{BinaryTree, Node, NodePtr};

///
/// Primary Implementation Method
///
/// Recursively split `ls` in three:
/// * The midpoint becomes a new node
/// * Entries to its left and right each generate children via recursive calls
///
fn minimal_tree_node(ls: &[isize]) -> Option<NodePtr<isize>> {
    if ls.is_empty() {
        // Base case: list has been exhausted.
        return None;
    }
    let mid = ls.len() / 2;
    let mut node = Node::new(ls[mid], None, None);
    node.left = minimal_tree_node(&ls[0..mid]);
    node.right = minimal_tree_node(&ls[mid + 1..ls.len()]);
    Some(NodePtr::new(node))
}
///
/// Top-Level Method
///
/// Take the result of `minimal_tree_node`, and make it the `head` of a new [BinaryTree].
///
pub fn minimal_tree(ls: &[isize]) -> BinaryTree<isize> {
    BinaryTree {
        head: minimal_tree_node(ls),
    }
}
#[test]
fn test_minimal_tree() {
    let test_array = [
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 18, 22, 43, 144, 515, 4123,
    ];
    let tree = minimal_tree(&test_array);

    // Checks on the result, which quickly turn quite gnarly.
    assert_eq!(tree.head.as_ref().unwrap().borrow().data, 10);
    assert_eq!(
        tree.head
            .as_ref()
            .unwrap()
            .borrow()
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .data,
        5
    );
    assert_eq!(
        tree.head
            .as_ref()
            .unwrap()
            .borrow()
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .data,
        43
    );
    // That's about enough of the literal checks.
    // Read out the in-order tree traversal, and check it matches `test_array`.
    let inorder = tree.inorder();
    assert_eq!(inorder, test_array);
}
