//!
//! # Remove Dups:
//!
//! Write code to remove duplicates from an unsorted li nked list.
//!
//! FOLLOW UP
//! How would you solve this problem if a temporary buffer is not allowed?
//!
//! Hints: #9, #40
//!

use std::collections::HashSet;
use std::hash::Hash;

// Local Imports
use super::utils::List;

///
/// Primary Implementation Method
///
/// Remove duplicates from a linked list.
/// Uses a [HashSet] to track previously-seen values.
/// Removal is delegated to [List::remove].
///
/// This solution *does not* use the "follow-up" method lacking intermediate storage,
/// which requires adding a two-pointer two-pass approach.  
///
pub fn remove_dups<T>(list: &mut List<T>)
where
    T: Clone + Hash + PartialEq + Eq,
{
    let mut seen = HashSet::new();
    let mut ptr = list.head;
    while let Some(idx) = ptr {
        let node = &list[idx];
        ptr = node.next;
        if seen.contains(&node.data) {
            list.remove(idx);
        } else {
            seen.insert(node.data.clone());
        }
    }
}

#[test]
fn test_remove_dups() {
    let test_cases = [
        (Vec::<i32>::new(), Vec::<i32>::new()),
        (vec![1, 1, 1, 1, 1, 1], vec![1]),
        (vec![1, 2, 3, 2], vec![1, 2, 3]),
        (vec![1, 2, 2, 3], vec![1, 2, 3]),
        (vec![1, 1, 2, 3], vec![1, 2, 3]),
        (vec![1, 2, 3], vec![1, 2, 3]),
    ];
    for case in test_cases {
        // Conver the test vector-data into a list
        let mut list = List::from(case.0.as_slice());
        // Run the solution
        remove_dups(&mut list);
        // And check the results
        assert_eq!(list.to_vec(), case.1);
    }
}
