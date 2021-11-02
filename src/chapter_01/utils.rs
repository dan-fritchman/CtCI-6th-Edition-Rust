//!
//! # Utility Module
//!
//! Shared among several problem-modules
//!

use std::collections::HashMap;

/// Create a mapping of character-counts in string `s`
pub fn char_counts(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

/// Create a mapping of *alphabetical* character-counts in string `s`.
/// Keys are all lower-case letters, values are the number of times each letter appears in `s`.
/// All non-alpha characters are not counted.
pub fn alpha_char_counts(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in s.chars() {
        if c.is_alphabetic() {
            *map.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
        }
    }
    map
}
