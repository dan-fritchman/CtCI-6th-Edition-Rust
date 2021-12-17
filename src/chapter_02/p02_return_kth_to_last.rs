//!
//! # Return Kth to Last:
//!
//! Implement an algorithm to find the kth to last element of a singly linked list.
//!
//! Hints: #8, #25, #47, #67, # 726
//!

// Local Imports
use super::utils::List;

///
/// Primary Implementation
///
/// Since this is a doubly-linked list, we can just count `k` elements from the tail.
/// Returns `None` if k is greater than the length of the list.
///
pub fn kth_to_last<T>(list: &List<T>, k: usize) -> Option<&T> {
    // Note `k==1` appears to want the tail.
    // So if we get k<1, return None, I guess.
    if k < 1 {
        return None;
    }
    let mut ptr = list.tail;
    for _ in 0..k - 1 {
        ptr = match ptr {
            Some(idx) => list[idx].prev,
            None => return None,
        };
    }
    // We've reached our destination. Wrap it in an Option, and return it.
    // Note the result can still be `None`, if k equals the length of the list
    match ptr {
        Some(idx) => Some(&list[idx].data),
        None => None,
    }
}

#[test]
fn test_kth_to_last() {
    let test_cases = [
        (vec![10, 20, 30, 40, 50], 1, 50),
        (vec![10, 20, 30, 40, 50], 5, 10),
    ];
    for case in test_cases {
        let mut list = List::from(case.0.as_slice());
        let kth = kth_to_last(&mut list, case.1).unwrap();
        assert_eq!(*kth, case.2);
    }
}
