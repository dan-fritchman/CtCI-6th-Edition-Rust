//!
//! # Parens:
//!
//! Implement an algorithm to print all valid (e.g., properly opened and closed) combinations of n pairs of parentheses.
//!
//! EXAMPLE
//!
//! Input: 3
//! Output: ["((()))", "(()())", "(())()", "()(())", "()()()"]
//!
//! Hints: # 138, #174, #187, #209, #243, #265, #295
//!

use std::collections::HashSet;

/// Primary Implementation
///
/// Recursively get the paren-combinations for `n-1`, and append an extra set in three places:
/// before, after, and surrounding.
///
/// The [HashSet] return type filters out any duplicates,
/// as well as dropping any seeming relevance of the item-order.
///
/// Recursion ends with zero or one set(s) of parens,
/// for which correct results are provided as [HashSet] literals.
///
pub fn parens(n: usize) -> HashSet<String> {
    match n {
        // Base cases: zero or one set of parens
        0 => HashSet::new(),
        1 => HashSet::from(["()".to_string()]),
        _ => {
            // Recursive case
            let mut set = HashSet::new();
            for s in parens(n - 1) {
                set.insert(format!("({})", s));
                set.insert(format!("(){}", s));
                set.insert(format!("{}()", s));
            }
            set
        }
    }
}

#[test]
fn test_parens() {
    let test_cases = [
        (0, vec![]),
        (1, vec!["()"]),
        (2, vec!["()()", "(())"]),
        (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
    ];
    for case in test_cases {
        // Convert the test-data vector-literal into an [HashSet]
        use std::iter::FromIterator;
        let correct = HashSet::from_iter(case.1.into_iter().map(|s| s.to_string()));

        // Run and check our results
        assert_eq!(parens(case.0), correct);
    }
}
