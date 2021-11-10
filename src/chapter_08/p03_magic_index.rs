//!
//! # Magic Index:
//!
//! A magic index in an array A[0...n-1] is defined to be an index such that A[i] = i.
//! Given a sorted array of distinct integers, write a method to find a magic index, if one exists, in array A.
//!
//! FOLLOW UP
//! What if the values are not distinct?
//!
//! Hints: # 170, #204, #240, #286, #340
//!

use std::cmp::Ordering;

pub fn magic_index(arr: &[isize]) -> Option<isize> {
    helper(arr, 0, arr.len() as isize)
}
pub fn helper(arr: &[isize], start: isize, end: isize) -> Option<isize> {
    if arr.is_empty() || end <= start || start < 0 || end < 0 {
        return None;
    }
    let mid = (start + end) / 2;
    let mididx = mid as usize;

    match arr[mididx].cmp(&mid) {
        Ordering::Equal => Some(mid),
        Ordering::Greater => {
            // Search right side
            if let Some(r) = helper(arr, mid + 1, end) {
                return Some(r);
            }
            // Search valid indices on right side
            let maxidx = arr[mididx].min(mid - 1);
            helper(arr, start, maxidx)
        }
        Ordering::Less => {
            // Search left side
            if let Some(r) = helper(arr, start, mid) {
                return Some(r);
            }
            // Search valid indices on right side
            let minidx = arr[mididx].max(mid + 1);
            helper(arr, minidx, end)
        }
    }
}

#[test]
fn test_magic_index() {
    let test_cases = [
        (vec![], None),
        (vec![0, 1, 2, 3, 4], Some(2)),
        (vec![-14, -12, 0, 1, 2, 5, 9, 10, 23, 25], Some(5)),
        (vec![-14, -12, 0, 1, 2, 7, 9, 10, 23, 25], None),
        (vec![-10, -5, 2, 2, 2, 3, 4, 7, 9, 12, 13], Some(2)),
    ];
    for case in test_cases {
        assert_eq!(magic_index(&case.0), case.1);
    }
}
