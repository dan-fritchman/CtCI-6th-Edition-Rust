//!
//! # Validate BST:
//!
//! Implement a function to check if a binary tree is a binary search tree.
//!
//! Hints: #35, #57, #86, # 773, # 728
//!

use crate::binary_tree::{BinaryTree, NodePtr};

/// Primary Implementation
///
/// Tracking a maximum and minimum valid values as we go, recurse down the tree,
/// checking that each sub-tree meets the BST properties.
///
pub fn validate_bst(tree: &BinaryTree<isize>) -> bool {
    helper(&tree.head, isize::MIN, isize::MAX)
}

/// Recursive helper.
/// Return whether `min` < `ptr.data` < `max`, and that each of `ptr`'s sub-trees also meets the BST properties.
fn helper(ptr: &Option<NodePtr<isize>>, min: isize, max: isize) -> bool {
    let ptr = match ptr {
        None => return true,
        Some(ref p) => p,
    };
    // Check that each sub-tree, and the node itself, meet the BST properties
    let node = ptr.borrow();
    node.data < max
        && node.data > min
        && helper(&node.left, min, node.data)
        && helper(&node.right, node.data, max)
}

#[test]
fn test_validate_bst() {
    use crate::binary_tree::Node;

    let tree = BinaryTree {
        head: Some(NodePtr::new(Node::new(
            20,
            Some(NodePtr::new(Node::new(
                9,
                Some(NodePtr::new(Node::new(5, None, None))),
                Some(NodePtr::new(Node::new(
                    12,
                    Some(NodePtr::new(Node::new(11, None, None))),
                    Some(NodePtr::new(Node::new(14, None, None))),
                ))),
            ))),
            Some(NodePtr::new(Node::new(25, None, None))),
        ))),
    };

    assert_eq!(validate_bst(&tree), true);

    let tree = BinaryTree {
        head: Some(NodePtr::new(Node::new(
            5,
            Some(NodePtr::new(Node::new(
                4,
                Some(NodePtr::new(Node::new(
                    3,
                    Some(NodePtr::new(Node::new(2, None, None))),
                    None,
                ))),
                Some(NodePtr::new(Node::new(6, None, None))),
            ))),
            Some(NodePtr::new(Node::new(
                6,
                Some(NodePtr::new(Node::new(5, None, None))),
                None,
            ))),
        ))),
    };

    assert_eq!(validate_bst(&tree), false);
}
