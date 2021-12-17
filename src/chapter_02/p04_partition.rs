//!
//! # Partition:
//!
//! Write code to partition a linked list around a value x, such that all nodes less than x come before all nodes greater than or equal to x.
//! If x is contained within the list, the values of x only need to be after the elements less than x (see below).
//! The partition element x can appear anywhere in the "right partition"; it does not need to appear between the left and right partitions.
//!
//! EXAMPLE
//! Input: 3 -> 5 -> 8 -> 5 ->10 -> 2 -> 1 [partition=5)
//! Output: 3 -> 1 -> 2 -> 10 -> 5 -> 5 -> 8
//!
//! Hints: #3, #24
//!

use std::cmp::PartialOrd;

// Local Imports
use super::utils::List;

pub fn partition<T: PartialOrd>(list: &mut List<T>, pivot: T) {
    // Create pointers into the new `left` and `right` lists
    let mut left = None;
    let mut right = None;
    // And maintain one to the first element in `right`
    let mut right_head = None;
    let mut ptr = list.head;
    while let Some(idx) = ptr {
        ptr = list[idx].next;
        if list[idx].data < pivot {
            // Add to the left list
            if let Some(lf) = left {
                list[lf].next = Some(idx);
                list[idx].prev = Some(lf);
            } else {
                // No left pointer yet. This becomes our new `head`.
                list.head = Some(idx);
                list[idx].prev = None;
            }
            left = Some(idx);
        } else {
            // Add to the right list, as the new tail.
            list[idx].next = None;
            if let Some(rt) = right {
                list[rt].next = Some(idx);
                list[idx].prev = Some(rt);
            } else {
                // No right head-pointer yet, set it
                right_head = Some(idx);
                list[idx].prev = None;
            }
            right = Some(idx);
        }
    }
    // Now link the left and right halves together
    if let Some(lf) = left {
        list[lf].next = right_head;
    } else {
        // Nothing in left, set the right to be the head
        list.head = right_head;
    }
    // And update the list-tail to the final element of `right`.
    list.tail = right;
}
#[test]
pub fn test_partition() {
    let mut list = List::from(vec![3, 5, 8, 5, 10, 2, 1].as_slice());
    partition(&mut list, 5);
    // Note this is *not* the value used in the example,
    // but another valid one produced by the "stable ordering" implementation above.
    assert_eq!(list.to_vec(), vec![3, 2, 1, 5, 8, 5, 10]);

    let mut list = List::from(vec![33, 11, 21, 5, 3, 4, 5].as_slice());
    partition(&mut list, 10);
    assert_eq!(list.to_vec(), vec![5, 3, 4, 5, 33, 11, 21]);
}
