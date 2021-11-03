//!
//! # One Away:
//!
//! There are three types of edits that can be performed on strings:
//! insert a character, remove a character, or replace a character.
//! Given two strings, write a function to check if they are one edit (or zero edits) away.
//!
//! EXAMPLE
//! pale, ple -> true
//! pales, pale -> true
//! pale, bale -> true
//! pale, bake -> false
//!
//! Hints: #23, #97, #130
//!

/// Primary Implementation Method
///
/// Split up handling between three cases: same-length, diff-by-one length, and
/// any other difference in length. The latter of which always returns false.
///
pub fn is_one_away(s1: &str, s2: &str) -> bool {
    // Hopefully the lengths of each of these strings fits in 64 bits, or we panic.
    let lendiff = (s1.len() as i64) - (s2.len() as i64);
    match lendiff {
        0 => same_len(s1, s2),      // Equal length - must be a replacement case
        1 => len_one_away(s1, s2),  // One more character in `s1`
        -1 => len_one_away(s2, s1), // One more character in `s2`
        _ => false,                 // Length differs by more than one, fail.
    }
}

/// Apply this test for strings of the same length.
/// Requires at most one character differs.
fn same_len(s1: &str, s2: &str) -> bool {
    let mut seen_diff = false;
    let mut s2chars = s2.chars();
    for c1 in s1.chars() {
        // `unwrap` here is safe, since we know the two strings are the same length
        let c2 = s2chars.next().unwrap();
        if c1 != c2 {
            if seen_diff {
                return false;
            }
            seen_diff = true;
        }
    }
    true
}

/// Apply the test for strings `longer` and `shorter` in which
/// one has been pre-verified to have length one greater than the other.
fn len_one_away(longer: &str, shorter: &str) -> bool {
    let mut seen_diff = false;
    let mut longer_iter = longer.chars();
    for shorter_char in shorter.chars() {
        if longer_iter.next().unwrap() != shorter_char {
            if seen_diff {
                return false;
            }
            // Set the "seen a difference" flag, skip over and check the next longer-iter character
            seen_diff = true;
            if longer_iter.next().unwrap() != shorter_char {
                return false;
            }
        }
    }
    true
}

#[test]
fn test_one_away() {
    let test_cases = [
        // no changes
        ("pale", "pale", true),
        ("", "", true),
        // one insert
        ("pale", "ple", true),
        ("ple", "pale", true),
        ("pales", "pale", true),
        ("ples", "pales", true),
        ("pale", "pkle", true),
        ("paleabc", "pleabc", true),
        ("", "d", true),
        ("d", "de", true),
        // one replace
        ("pale", "bale", true),
        ("a", "b", true),
        ("pale", "ble", false),
        // multiple replace
        ("pale", "bake", false),
        // insert and replace
        ("pale", "pse", false),
        ("pale", "pas", false),
        ("pas", "pale", false),
        ("pkle", "pable", false),
        ("pal", "palks", false),
        ("palks", "pal", false),
        // permutation with insert shouldn't match
        ("ale", "elas", false),
    ];
    for case in test_cases {
        assert_eq!(is_one_away(case.0, case.1), case.2);
    }
}
