//!
//! # Check Balanced:
//!
//! Implement a function to check if a binary tree is balanced.
//! For the purposes of this question, a balanced tree is defined to be a tree such that
//! the heights of the two subtrees of any node never differ by more than one.
//!
//! Hints: #27, #33, #49, #705, #724
//!

use crate::binary_tree::{BinaryTree, NodePtr};

/// Primary Implementation
///
/// Recursively check the balance and height of each sub-tree.
///
pub fn check_balanced(tree: &BinaryTree<isize>) -> bool {
    // Kick off recursive calls.
    //
    // While our helper also returns the tree-height if balanced,
    // we only care about the boolean indication of its balance,
    // indicated by its returning `Some(_)`.
    helper(&tree.head).is_some()
}

/// Recursive helper.
/// Returns the height of the tree beginning with `ptr`, if that sub-tree is balanced.
/// Returns `None` if it is not balanced.
fn helper(ptr: &Option<NodePtr<isize>>) -> Option<usize> {
    // Get the contents of our node-pointer, or return if null
    let ptr = match ptr {
        None => return Some(0), // Base case: no node
        Some(ref p) => p,       // Node here, keep going
    };

    // Get the heights of left and right sub-trees, if balanced.
    // Note the question-mark operators early-return `None` if not.
    let node = ptr.borrow();
    let left_depth = helper(&node.left)?;
    let right_depth = helper(&node.right)?;

    // And sort out whether the two are balanced
    let max = left_depth.max(right_depth);
    let min = left_depth.min(right_depth);
    let rv = if max - min < 2 { Some(max + 1) } else { None };
    rv
}

#[test]
fn test_check_balanced() {
    use crate::binary_tree::Node;

    let balanced1 = || {
        let left = NodePtr::new(Node::new(2, None, None));
        let head = Some(NodePtr::new(Node::new(1, Some(left.clone()), None)));
        BinaryTree { head }
    };
    let balanced2 = || {
        let ten = NodePtr::new(Node::new(10, None, None));
        let nine = NodePtr::new(Node::new(9, None, Some(ten.clone())));
        let eight = NodePtr::new(Node::new(8, None, None));
        let three = NodePtr::new(Node::new(3, Some(eight.clone()), Some(nine.clone())));
        let four = NodePtr::new(Node::new(4, None, None));
        let two = NodePtr::new(Node::new(2, Some(four.clone()), None));
        let head = Some(NodePtr::new(Node::new(
            7,
            Some(two.clone()),
            Some(three.clone()),
        )));
        BinaryTree { head }
    };
    let unbalanced1 = || {
        let elev = NodePtr::new(Node::new(11, None, None));
        let ten = NodePtr::new(Node::new(10, None, Some(elev.clone())));
        let nine = NodePtr::new(Node::new(9, None, Some(ten.clone())));
        let seven = NodePtr::new(Node::new(7, None, None));
        let six = NodePtr::new(Node::new(6, None, Some(seven.clone())));
        let five = NodePtr::new(Node::new(5, None, Some(six.clone())));
        let four = NodePtr::new(Node::new(4, None, None));
        let eight = NodePtr::new(Node::new(8, None, None));
        let three = NodePtr::new(Node::new(3, Some(eight.clone()), Some(nine.clone())));
        let two = NodePtr::new(Node::new(2, Some(four.clone()), Some(five.clone())));
        let head = Some(NodePtr::new(Node::new(
            1,
            Some(two.clone()),
            Some(three.clone()),
        )));
        BinaryTree { head }
    };
    let unbalanced2 = || {
        let sixteen = NodePtr::new(Node::new(16, None, None));
        let zero = NodePtr::new(Node::new(0, None, None));
        let twelve = NodePtr::new(Node::new(12, Some(sixteen.clone()), Some(zero.clone())));
        let six = NodePtr::new(Node::new(6, None, None));
        let five = NodePtr::new(Node::new(5, None, None));
        let seven = NodePtr::new(Node::new(7, Some(twelve.clone()), Some(five.clone())));
        let three = NodePtr::new(Node::new(3, Some(six.clone()), None));
        let ten = NodePtr::new(Node::new(10, None, None));
        let nine = NodePtr::new(Node::new(9, Some(ten.clone()), None));
        let two = NodePtr::new(Node::new(2, Some(three.clone()), Some(seven.clone())));
        let head = Some(NodePtr::new(Node::new(
            1,
            Some(two.clone()),
            Some(nine.clone()),
        )));
        BinaryTree { head }
    };

    let test_cases = [
        (balanced1(), true),
        (balanced2(), true),
        (unbalanced1(), false),
        (unbalanced2(), false),
    ];
    for case in test_cases {
        dbg!(&case.0);
        assert_eq!(check_balanced(&case.0), case.1);
    }
}
