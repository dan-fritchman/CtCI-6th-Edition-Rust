//!
//! # Palindrome Permutation:
//!
//! Given a string, write a function to check if it is a permutation of a palindrome.
//! A palindrome is a word or phrase that is the same forwards and backwards.
//! A permutation is a rearrangement of letters.
//! The palindrome does not need to be limited to just dictionary words.
//!
//! EXAMPLE
//! Input: Tact Coa
//! Output: True (permutations: "taco cat". "atco cta". etc.)
//! Hints: #106, #121, #134, #136
//!

use super::utils::alpha_char_counts;

/// Primary Solution Method
///
/// Uses the insight that palindromes are primarily built of character-pairs,
/// except for at most one character in the middle.
/// Creates a count of each (alpha) character, iterates over the counts,
/// and returns false if more than a single count is odd.
///
pub fn is_palindrome_permutation(s: &str) -> bool {
    let char_counts = alpha_char_counts(s);
    let mut seen_an_odd_one = false;
    for (_c, cnt) in char_counts {
        if cnt % 2 == 1 {
            if seen_an_odd_one {
                return false;
            }
            seen_an_odd_one = true;
        }
    }
    // Survived! At most one character has an odd count.
    // We can permute into a palindrome.
    true
}

#[test]
fn test_palindrome_permutation() {
    // Test cases, ported from prior versions
    let test_cases = [
        ("aba", true),
        ("aab", true),
        ("abba", true),
        ("aabb", true),
        ("a-bba", true),
        ("Tact Coa", true),
        ("jhsabckuj ahjsbckj", true),
        ("Able was I ere I saw Elba", true),
        ("So patient a nurse to nurse a patient so", false),
        ("Random Words", false),
        ("Not a Palindrome", false),
        ("no x in nixon", true),
        ("azAZ", true),
    ];
    for case in test_cases {
        assert_eq!(is_palindrome_permutation(&case.0), case.1);
    }
}
