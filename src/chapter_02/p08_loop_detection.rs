//!
//! # Loop Detection:
//!
//! Given a circular linked list, implement an algorithm that returns the node at the beginning of the loop.
//!
//! DEFINITION
//!
//! Circular linked list: A (corrupt) linked list in which a node's next pointer points to an earlier node, so as to make a loop in the linked list.
//!
//! EXAMPLE
//! Input: A -> B -> C -> D -> E -> C (the same `C` as earlier)
//! Output: C
//!
//! Hints: #50, #69, #83, #90
//!

use std::collections::HashSet;

// Local Imports
use super::utils::{List, NodeIndex};

pub fn loop_detect<T: Default>(list: &List<T>) -> Option<NodeIndex> {
    let mut seen: HashSet<NodeIndex> = HashSet::new();
    let mut ptr = list.head;
    while let Some(idx) = ptr {
        if seen.contains(&idx) {
            return Some(idx);
        }
        seen.insert(idx);
        ptr = list[idx].next;
    }
    None // No loops found
}

#[test]
fn test_loop_detect() {
    // Our implementation requires some semi-illegal fuddling to even *create* this illegal list.
    // Start with the valid part.
    let mut list = List::from(vec!['a', 'b', 'c', 'd', 'e'].as_slice());
    // Check that it doesn't find any loops.
    assert_eq!(loop_detect(&list), None);

    // And now do the illegal cheating. Give node-4 a `next` pointer back to node-2.
    list[NodeIndex(4)].next = Some(NodeIndex(2));
    // Note this also invalidates our `tail` pointer, although it's not clear what such
    // a thing could mean anyway in a list with loops.
    assert_eq!(loop_detect(&list), Some(NodeIndex(2)));
}
