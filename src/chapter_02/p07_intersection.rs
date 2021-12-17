//!
//! # Intersection:
//!
//! Given two (singly) linked lists, determine if the two lists intersect.
//! Return the intersecting node. Note that the intersection is defined based on reference, not value.
//! That is, if the kth node of the first linked list is the exact same node (by reference)
//! as the jth node of the second linked list, then they are intersecting.
//!
//! Hints: #20, #45, #55, #65, #76, #93, #111, #120, #129
//!

use std::collections::HashSet;

// We can't really do this one with our "slot-map-style" Rustic implementation.
// So, pull in the shared-pointer [Ptr] based [List] instead.

use crate::ptr::Ptr;
use crate::ptr_list::{List, Node};

/// Primary Implementation
///
/// Insert each element of `a` into a [HashSet],
/// then check whether it contains any elements of `b`.
///
pub fn intersection(a: &List, b: &List) -> Option<Ptr<Node>> {
    // Insert each node in `a` into a hash-set
    let mut aset = HashSet::new();

    // (How cool is that `impl Iterator` now.)
    for ptr in a.iter() {
        aset.insert(ptr.clone());
    }

    // And check whether it contains each element in `b`
    for ptr in b.iter() {
        if aset.contains(&ptr) {
            return Some(ptr);
        }
    }
    None // No intersection found
}

#[test]
fn test_intersecting() {
    // Create two (initially non-intersecting) lists
    let mut a = List::from_slice(&[10, 11, 12, 13, 14, 15]);
    let mut b = List::from_slice(&[20, 21, 22]);

    // Check that they do not intersect
    assert_eq!(intersection(&a, &b), None);

    // Now create a shared sub-list
    let mut shared = List::default();
    let shared_head = shared.add(1);
    shared.add_slice(&[2, 3, 4]);

    // (Illegally) link `shared` to the end of both `a` and `b`
    a.tail.as_mut().unwrap().borrow_mut().next = shared.head.clone();
    a.tail = shared.tail.clone();
    b.tail.as_mut().unwrap().borrow_mut().next = shared.head.clone();
    b.tail = shared.tail.clone();

    // Check the head of `shared` comes back as the result
    assert_eq!(intersection(&a, &b), Some(shared_head));
}
