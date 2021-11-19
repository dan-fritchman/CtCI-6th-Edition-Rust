//!
//! # Power Set:
//!
//! Write a method to return all subsets of a set.
//!
//! Hints: #273, #290, #338, #354, #373
//!

use std::collections::HashSet;

pub fn power_set(_set: &HashSet<isize>) -> Vec<Vec<isize>> {
    todo!()
}
#[ignore]
#[test]
fn test_power_set() {
    let test_cases = [(
        HashSet::from([1, 2, 3]),
        vec![
            vec![],
            vec![1],
            vec![1, 2],
            vec![1, 2, 3],
            vec![1, 3],
            vec![2],
            vec![2, 3],
            vec![3],
        ],
    )];
    for case in test_cases {
        assert_eq!(power_set(&case.0), case.1);
    }
}
