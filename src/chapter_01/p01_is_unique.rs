//!
//! # Is Unique:
//!
//! Implement an algorithm to determine if a string has all unique characters.  
//! What if you cannot use additional data structures?
//!
//! Hints: #44, # 777, # 732
//!

use std::collections::HashSet;

///
/// Primary string "character uniqueness" function
///
/// Operates by createing a hash-set of characters, inserting each character in `s`,
/// and comparing the size (length) of `s` with that of the hash-set.
/// Any duplicates will result in the set having fewer elements than the input string.
///
pub fn is_unique(s: &str) -> bool {
    let mut set = HashSet::new();
    for c in s.chars() {
        set.insert(c);
    }
    set.len() == s.len()
}

#[test]
fn test_is_unique() {
    // Initial cases
    assert_eq!(is_unique("abcdefg"), true);
    assert_eq!(is_unique("abcdefgabcdefg"), false);

    // List of cases ported from prior solutions
    let test_cases = [
        ("abcd", true),
        ("s4fad", true),
        ("", true),
        ("23ds2", false),
        ("hb 627jh=j ()", false),
        // ("".join([chr(val) for val in range(128)]), true),  # unique 128 chars
        // ("".join([chr(val // 2) for val in range(129)]), False),  # non-unique 129 chars
    ];
    for (s, expected) in test_cases {
        assert_eq!(is_unique(s), expected);
    }
}
