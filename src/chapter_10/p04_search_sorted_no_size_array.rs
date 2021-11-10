//!
//! # Sorted Search, No Size:
//!
//! You are given an array-like data structure Listy which lacks a size method.
//! It does, however, have an elementAt(i) method that returns the element at index i in 0(1) time.
//! If i is beyond the bounds of the data structure, it returns -1.
//! (For this reason, the data structure only supports positive integers.)
//! Given a Listy which contains sorted, positive integers,
//! find the index at which an element x occurs.
//! If x occurs multiple times, you may return any index.
//!
//! Hints: #320, #337, #348
//!

use std::cmp::Ordering;

pub struct Listy(Vec<isize>);
impl Listy {
    /// Get the element value at index `i`
    fn element_at(&self, i: usize) -> isize {
        if i > self.0.len() - 1 {
            -1
        } else {
            self.0[i]
        }
    }
}
pub fn sorted_search_no_size(listy: &Listy, x: isize) -> isize {
    // First binary-search-style figure out `listy`s length
    let (mut low, mut high) = (0, 1_000_000);
    while low < high - 1 {
        let test = (high + low) / 2;
        if listy.element_at(test as usize) == -1 {
            high = test;
        } else {
            low = test;
        }
    }
    // The converged result is `listy`s length.
    // Now do a regular binary search for `x`.
    let (mut low, mut high) = (0, high - 1);
    while low <= high {
        // Get the mid-point element
        let index = (high + low) / 2;
        let elem = listy.element_at(index as usize);

        // If it matches our target, return it. Otherwise, chop our range in half.

        // This code-block is one of the `clippy` linter's very examples:
        // https://rust-lang.github.io/rust-clippy/master/index.html#comparison_chain
        match elem.cmp(&x) {
            Ordering::Equal => return index,
            Ordering::Greater => high = index - 1,
            Ordering::Less => low = index + 1,
        };
    }
    -1 // Not found.
}

#[test]
fn test_sorted_search_no_size() {
    let test_cases = [
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 0), -1),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 1), 0),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 2), 1),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 3), 2),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 4), 3),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 5), 4),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 6), 5),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 7), 6),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 8), 7),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 9), 8),
        ((vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 10), -1),
    ];
    for case in test_cases {
        let listy = Listy(case.0 .0);
        assert_eq!(sorted_search_no_size(&listy, case.0 .1), case.1);
    }
}
