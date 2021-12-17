//!
//! # Permutations without Dups:
//!
//! Write a method to compute all permutations of a string of unique characters.
//!
//! Hints: #150, #185, #200, #267, #278, #309, #335, #356
//!

/// Primary Implementation
///
/// Peel out a character at a time, get all permutations of the rest, and append them to the character.
///
pub fn permutations_without_dups(s: &str) -> Vec<String> {
    if s.len() == 0 {
        return vec![String::new()]; // Base case. Include the empty string so calls above can append to it.
    }
    // Recursive case.
    // Peel out a character at a time, get all permutations of the rest, and append them to the character.
    let mut result = Vec::new();
    for idx in 0..s.len() {
        // Pull out the char at `idx`
        let rest = format!("{}{}", &s[0..idx], &s[idx + 1..s.len()]);
        // Get all permutations of the remaining characters
        let partials = permutations_without_dups(&rest);
        // And prepend the character at `idx` to them.
        for p in partials {
            result.push(format!("{}{}", &s[idx..idx + 1], p));
        }
    }
    result
}

#[test]
fn test_permutations_without_dups() {
    let test_cases = [
        ("a", vec!["a"]),
        ("ab", vec!["ab", "ba"]),
        ("abc", vec!["abc", "acb", "bac", "bca", "cab", "cba"]),
    ];
    for case in test_cases {
        assert_eq!(permutations_without_dups(case.0), case.1);
    }
}
