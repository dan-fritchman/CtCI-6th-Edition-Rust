//!
//! # String Rotation:
//!
//! Assume you have a method `isSubstring` which checks if one word is asubstring of another.
//! Given two strings, S1 and S2, write code to check if S2 is a rotation of S1 using only one call to isSubstring
//! (e.g.,"waterbottle"is a rotation of"erbottlewat").
//!
//! Hints: #34, #88, #104
//!

/// Standard-library implementation of the `isSubstring` method.
/// Returns whether `a` is a sub-string of `b`.
fn is_substr(a: &str, b: &str) -> bool {
    b.contains(a)
}

/// Primary Implementation
///
/// Check whether `s2` is a rotation of `s1`, primarily by checking whether `s2` is a sub-string of twice-repeated `s1s1`.
///
pub fn is_str_rotation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    is_substr(s2, &s1.repeat(2))
}

#[test]
fn test_str_rotation() {
    let test_cases = [
        ("waterbottle", "erbottlewat", true),
        ("foo", "bar", false),
        ("foo", "foofoo", false),
    ];
    for case in test_cases {
        assert_eq!(is_str_rotation(case.0, case.1), case.2);
    }
}
