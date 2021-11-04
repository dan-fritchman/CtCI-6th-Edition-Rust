//!
//! # Delete Middle Node:
//!
//! Implement an algorithm to delete a node in the middle
//! (i.e., any node but the first and last node, not necessarily the exact middle) of a singly linked list,
//! given only access to that node.
//!
//! EXAMPLE
//! Input: the node c from the linked list a -> b -> c -> d -> e -> f
//! Result: nothing is returned, but the new linked list looks like a -> b -> d -> e -> f
//!
//! Hints: #72
//!

// Local Imports
use super::utils::{List, NodeIndex};

///
/// Primary Implementation Method
///
/// The whole Rustic approach of our linked list - a vector with index "pointers" -
/// kinda prevents this problem from being done as intended.
/// The intent seems to be to write a function of the form `fn delele_node(node: &mut Node)`
/// which *does not* have access to the rest of the list, notably the head pointer.
///
/// In our implementation the [List] *owns* its nodes. There's no way to have a "free node" which is also in a list,
/// nor to even get a reference to a node in a list, while retaining the ability to modify the list.
///
/// *But!* we can do some stuff the target implementation can't.
/// Notably, deleting first and last elements in the list is fine here.
///
pub fn delete_node<T: Default>(list: &mut List<T>, remove: NodeIndex) {
    match (list[remove].prev, list[remove].next) {
        (Some(prev), Some(next)) => {
            // Node is in the middle, not the head or tail
            list[prev].next = list[remove].next;
            list[next].prev = list[remove].prev;
        }
        (None, Some(next)) => {
            // Node is the head, but not the tail
            list[next].prev = None;
            list.head = Some(next);
        }
        (Some(prev), None) => {
            // Node is the tail, but not the head
            list[prev].next = None;
            list.tail = Some(prev);
        }
        (None, None) => {
            // Only element in the list. Both pointers become null.
            list.head = None;
            list.tail = None;
        }
    }
}

#[test]
fn test_delete_node() {
    let test_cases = [
        ([0, 1, 2, 3, 4, 5], NodeIndex(0), [1, 2, 3, 4, 5]),
        ([0, 1, 2, 3, 4, 5], NodeIndex(2), [0, 1, 3, 4, 5]),
        ([0, 1, 2, 3, 4, 5], NodeIndex(4), [0, 1, 2, 3, 5]),
    ];
    for case in test_cases {
        let mut list = List::from(case.0.as_ref());
        delete_node(&mut list, case.1);
        assert_eq!(list.to_vec(), case.2);
    }
}
