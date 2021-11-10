//!
//! # Sorted Merge:
//!
//! You are given two sorted arrays, A and B, where A has a large enough buffer at the end to hold B.
//! Write a method to merge B into A in sorted order.
//!
//! Hints: #332
//!

/// Primary Implementation
///
/// The primary "trick" here is "going backwards", starting from the *ends* of `a` and `b`.
///
/// This problem is another example of where Rust's signed/ unsigned integer rules,
/// particularly for indexing into arrays, seem to get in the way.
///
pub fn sorted_merge(a: &mut [isize], b: &[isize]) {
    if b.len() >= a.len() {
        panic!("Invalid Inputs");
    }
    let mut merge_idx = a.len() - 1;
    let mut b_idx = (b.len() - 1) as isize;
    let mut a_idx = (a.len() - b.len() - 1) as isize;

    while b_idx >= 0 {
        if a_idx >= 0 && a[a_idx as usize] >= b[b_idx as usize] {
            a[merge_idx] = a[a_idx as usize];
            a_idx -= 1;
        } else {
            a[merge_idx] = b[b_idx as usize];
            b_idx -= 1;
        }
        if merge_idx > 0 {
            merge_idx -= 1;
        }
    }
    // Done - any remaining `a` elements are already in place.
}

#[test]
fn test_sorted_merge() {
    let mut a = vec![9, 10, 11, 12, 13, 0, 0, 0, 0];
    let b = vec![4, 5, 6, 7];
    let expected = vec![4, 5, 6, 7, 9, 10, 11, 12, 13];
    sorted_merge(&mut a, &b);
    assert_eq!(a, expected);
}
