//!
//! # Check Permutation:
//!
//! Given two strings, write a method to decide if one is a permutation of the other.
//! Hints: #7, #84, #722, #737
//!

// Local Imports
use super::utils::char_counts;

/// Primary Method
///
/// Collect character-counts in a hashmap, and compare the two.
///
#[allow(dead_code)]
fn is_permutation(a: &str, b: &str) -> bool {
    // Before getting into the real work, check lengths.
    if a.len() != b.len() {
        return false;
    }

    // Get the per-character counts for each
    let acounts = char_counts(a);
    let bcounts = char_counts(b);

    // And compare the two
    acounts == bcounts
}

#[test]
fn test_is_permutation() {
    let test_cases = [
        ("dog", "god", true),
        ("abcd", "bacd", true),
        ("3563476", "7334566", true),
        ("wef34f", "wffe34", true),
        ("dogx", "godz", false),
        ("abcd", "d2cba", false),
        ("2354", "1234", false),
        ("dcw4f", "dcw5f", false),
        ("DOG", "dog", false),
        ("dog ", "dog", false),
        ("aaab", "bbba", false),
    ];
    for (a, b, expected) in test_cases {
        assert_eq!(is_permutation(a, b), expected);
    }
}
