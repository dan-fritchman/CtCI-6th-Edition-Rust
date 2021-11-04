//!
//! # Intersection:
//!
//! Given two (singly) linked lists, determine if the two lists intersect.
//! Return the intersecting node. Note that the intersection is defined based on reference, not value.
//! That is, if the kth node of the first linked list is the exact same node (by reference) as the jth node of the second linked list, then they are intersecting.
//!
//! Hints: #20, #45, #55, #65, #76, #93, #111, #120, #129
//!

///
/// Well, we can't really do this one with our "slot-map-style" Rustic implementation.
///
/// Our Lists own their Nodes - there's no "free floating" Node possible which could be part of two different Lists.
/// If such a thing got created somehow, we'd just interpret its index-pointers as being relative to
/// whichever List owned it.
///
/// If we re-do another version with `RefCell` or similar, do it singly-linked, and
/// we should be able to do this exercise (and make a few others more interesting).
///
#[test]
fn test_intersecting() {
    dbg!("🤷");
}
