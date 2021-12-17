//!
//! # Power Set:
//!
//! Write a method to return all subsets of a set.
//!
//! Hints: #273, #290, #338, #354, #373
//!

/// Primary Implementation
///
/// Recursively peel off a value at a time, get the power set of the remainder,
/// clone that remainder and add the final value to each entry.
///
pub fn power_set(set: &[isize]) -> Vec<Vec<isize>> {
    if set.is_empty() {
        return vec![vec![]]; // Base case: empty set
    }
    // Recursive cases. Pop the last element, get the sets of each one-shorter set.
    let mut children = power_set(&set[0..set.len() - 1]);
    let mut clones = children.clone();
    for clone in clones.iter_mut() {
        clone.push(set[set.len() - 1]);
    }
    children.extend(clones);
    children
}

#[test]
fn test_power_set() {
    // Note that unlike in the spirit of the problem,
    // these have to come back in the expected *order* to be correct.
    // Changing them all to [HashSet] would remove this constraint,
    // but the implementation above is designed to meet it.
    let test_cases = [
        (vec![], vec![vec![]]),
        (vec![11], vec![vec![], vec![11]]),
        (vec![22, 33], vec![vec![], vec![22], vec![33], vec![22, 33]]),
        (
            vec![1, 2, 3],
            vec![
                vec![],
                vec![1],
                vec![2],
                vec![1, 2],
                vec![3],
                vec![1, 3],
                vec![2, 3],
                vec![1, 2, 3],
            ],
        ),
    ];
    for case in test_cases {
        assert_eq!(power_set(&case.0), case.1);
    }
}
