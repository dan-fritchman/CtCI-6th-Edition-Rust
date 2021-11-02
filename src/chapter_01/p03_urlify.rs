//!
//! # URLify:
//! 
//! Write a method to replace all spaces in a string with '%20'.
//! You may assume that the string has sufficient space at the end to hold the additional characters,
//! and that you are given the "true" length of the string.
//! (Note: If implementing in Java, please use a character array so that you can perform this operation in place.)
//!
//! EXAMPLE
//! Input: "Mr John Smith " J 13
//! Output: "Mr%20John%20Smith"
//!
//! Hints: #53, #7 78
//!

/// Primary solution method
///
/// Iterates over characters, replacing white-spaces with the string "%20".
/// Allocates and returns a new string.
///
/// While unclear from the problem statement, alternately available solutions
/// appear to target truncating the result-length to `truelen`.
///
/// It is not entirely clear (to the author) whether consecutive spaces should generate one or more "%20" sequences.
/// This implementation replaces each space with a unique "%20".
///
#[allow(dead_code)]
fn urlify(s: &str, truelen: usize) -> String {
    let mut rv = String::new();
    let mut chars = s.chars();
    for _ in 0..truelen {
        // Here `unwrap`ing the next character is (promised) safe by the problem statement:
        // We will have enough characters to fill `truelen` (or we'll panic).
        let c = chars.next().unwrap();
        if c.is_ascii_whitespace() {
            rv.push_str("%20");
        } else {
            rv.push(c);
        }
    }
    rv
}

#[test]
fn test_urlify() {
    // Test cases, derived from prior versions
    let test_cases = [
        (
            "much ado about nothing      ",
            22,
            "much%20ado%20about%20nothing",
        ),
        ("Mr John Smith       ", 13, "Mr%20John%20Smith"),
        (" a b    ", 4, "%20a%20b"),
        (" a b       ", 5, "%20a%20b%20"),
    ];
    for case in test_cases {
        // Note `case.1`, the "true length", is unused.
        assert_eq!(urlify(&case.0, case.1), case.2);
    }
}
