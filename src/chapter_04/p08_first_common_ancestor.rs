//!
//! # First Common Ancestor:
//!
//! Design an algorithm and write code to find the first common ancestor of two nodes in a binary tree. Avoid storing additional nodes in a data structure.
//! NOTE: This is not necessarily a binary search tree.
//!
//! Hints: # 10, #16, #28, #36, #46, #70, #80, #96
//!

use crate::binary_tree::{BinaryTree, NodePtr};

/// Primary Implementation
///
/// This solution uses a method similar to CtCI's "Solution #4 - Optimizied",
/// from among solutions *without* parent-pointers.
/// It is modified to use a Rust-idiomatic `enum` type to indicate results of searches from sub-trees.
///
pub fn first_common_ancestor(
    tree: &BinaryTree<isize>,
    p: &NodePtr<isize>,
    q: &NodePtr<isize>,
) -> Option<NodePtr<isize>> {
    // Search for the two nodes, generating a [SubTree] result
    let subtree = helper(&tree.head, p, q);

    // And return the ancestor-node, only if it found both
    if let SubTree::HasBoth(a) = subtree {
        Some(a)
    } else {
        None
    }
}

/// Recursive helper
///
/// Create a [SubTree] result indicating either a common ancestor, or the presence/ absence of `p` and `q`.
fn helper(root: &Option<NodePtr<isize>>, p: &NodePtr<isize>, q: &NodePtr<isize>) -> SubTree {
    use SubTree::{HasBoth, HasNeither, HasP, HasQ};

    // Unwrap the root-node pointer-option. Return neither-found if it is `None`.
    let root = match root {
        None => return HasNeither,
        Some(ref r) => r,
    };

    // Search the node's left and right sub-trees
    let node = root.borrow();
    let left = helper(&node.left, p, q);
    let right = helper(&node.right, p, q);

    // And combine the two
    match (left, right) {
        (HasBoth(n), _) | (_, HasBoth(n)) => HasBoth(n), // Found in a sub-tree
        (HasQ, HasP) | (HasP, HasQ) => HasBoth(root.clone()), // `root` is it
        (HasP, HasNeither) | (HasNeither, HasP) => {
            // `p` was found in a sub-tree. Convert to `HasBoth` if root is `q`.
            if root == q {
                HasBoth(root.clone())
            } else {
                HasP
            }
        }
        (HasQ, HasNeither) | (HasNeither, HasQ) => {
            // `q` was found in a sub-tree. Convert to `HasBoth` if root is `p`.
            if root == p {
                HasBoth(root.clone())
            } else {
                HasQ
            }
        }
        (HasNeither, HasNeither) => {
            // Neither found. Check `root`
            if root == q {
                HasQ
            } else if root == p {
                HasP
            } else {
                HasNeither
            }
        }
        _ => unreachable!("Internal Error"),
    }
}
/// Enumerated Results for searching a sub-tree
#[derive(Debug, PartialEq, Eq)]
pub enum SubTree {
    HasNeither,
    HasP,
    HasQ,
    HasBoth(NodePtr<isize>),
}

#[test]
fn test_first_common_ancestor() {
    use crate::binary_tree::Node;

    let n8 = NodePtr::new(Node::new(8, None, None));
    let n7 = NodePtr::new(Node::new(8, None, None));
    let n5 = NodePtr::new(Node::new(5, None, None));
    let n4 = NodePtr::new(Node::new(4, Some(n8.clone()), None));
    let n3 = NodePtr::new(Node::new(3, None, Some(n7.clone())));
    let n2 = NodePtr::new(Node::new(2, Some(n4.clone()), Some(n5.clone())));
    let n1 = NodePtr::new(Node::new(1, Some(n2.clone()), Some(n3.clone())));
    let tree = BinaryTree {
        head: Some(n1.clone()),
    };

    assert_eq!(first_common_ancestor(&tree, &n3, &n4), Some(n1));
}
