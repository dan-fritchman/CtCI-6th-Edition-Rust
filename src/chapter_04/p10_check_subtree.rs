//!
//! # Check Subtree:
//!
//! T1 and T2 are two very large binary trees, with T1 much bigger than T2.
//! Create an algorithm to determine if T2 is a subtree of T1.
//!
//! A tree T2 is a subtree ofT i if there exists a node n in T i such that the subtree of n is identical to T2.
//! That is, if you cut off the tree at node n, the two trees would be identical.
//!
//! Hints: #4, #77, #78, #37, #37
//!

#[cfg(test)]
use crate::binary_tree::Error;
use crate::binary_tree::{BinaryTree, NodePtr};

/// Primary Implementation
///
/// Search `t1` for nodes equal to `t2`'s root.
/// Upon finding one, compare all remaining nodes in their sub-trees.
///
pub fn check_subtree<T: Eq>(t1: &BinaryTree<T>, t2: &BinaryTree<T>) -> bool {
    search(&t1.head, &t2.head)
}
// Search for `targ`'s root-node. Then compare sub-trees the rest of the way down.
fn search<T: Eq>(tree: &Option<NodePtr<T>>, targ: &Option<NodePtr<T>>) -> bool {
    match (tree.as_ref(), targ.as_ref()) {
        (Some(tr), Some(ta)) => {
            // If we hit the root value, compare the rest
            if tr.borrow().data == ta.borrow().data && compare(tree, targ) {
                true
            } else {
                // Otherwise, keep searching for it.
                search(&tr.borrow().left, targ) || search(&tr.borrow().right, targ)
            }
        }
        (_, None) => true,  // Null nodes count as a sub-tree of any tree
        (None, _) => false, // No tree to search, no hits
    }
}
/// Compare (sub)trees `t1` and `t2`
fn compare<T: Eq>(t1: &Option<NodePtr<T>>, t2: &Option<NodePtr<T>>) -> bool {
    match (t1.as_ref(), t2.as_ref()) {
        (Some(t1), Some(t2)) => {
            t1.borrow().data == t2.borrow().data // Two nodes must be equal, and 
                && compare(&t1.borrow().left, &t2.borrow().left)   // Left sub-tree must be equal, and 
                && compare(&t1.borrow().right, &t2.borrow().right) // Right sub-tree must be equal
        }
        (None, None) => true, // Both null is a match
        _ => false,           // Either other case (Some, None) | (None, Some) is a mismatch
    }
}

#[test]
fn test_check_subtree() -> Result<(), Error> {
    let mut t1 = BinaryTree::<isize>::default();
    let n1 = t1.insert(1, None)?;
    let n2 = t1.insert(2, Some(n1.clone()))?;
    let n3 = t1.insert(3, Some(n1))?;
    let n4 = t1.insert(4, Some(n2.clone()))?;
    let _5 = t1.insert(5, Some(n2))?;
    let _7 = t1.insert(7, Some(n3))?;
    let n8 = t1.insert(8, Some(n4))?;

    let mut t2 = BinaryTree::default();
    let n40 = t2.insert(4, None)?;
    let _80 = t2.insert(8, Some(n40))?;

    assert_eq!(check_subtree(&t1, &t2), true); // Equal but non-identical sub-trees

    let t3 = BinaryTree {
        head: Some(n8.clone()),
    };
    assert_eq!(check_subtree(&t1, &t3), true); // Identity-equal sub-tree

    let mut t4 = BinaryTree::default();
    t4.insert(111, None)?;
    assert_eq!(check_subtree(&t1, &t4), false);

    Ok(())
}
