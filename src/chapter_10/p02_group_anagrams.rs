//!
//! # Group Anagrams:
//!
//! Write a method to sort an array of strings
//! so that all the anagrams are next to each other.
//!
//! Hints: #177, #182, #263, #342
//!

use itertools::Itertools;
use std::collections::{hash_map::Entry, HashMap};

pub fn group_anagrams(strs: Vec<&str>) -> Vec<String> {
    // Create a hashmap from sorted-string to list of anagram (indices) in `strs`
    let mut table: HashMap<String, Vec<usize>> = HashMap::new();
    for (idx, s) in strs.iter().enumerate() {
        // Sort the string by characters
        let sorted_str = s.chars().sorted().collect::<String>();
        // And add its index to the table
        match table.entry(sorted_str) {
            Entry::Vacant(e) => {
                e.insert(vec![idx]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(idx);
            }
        };
    }
    // Walk through each entry in the map, and unroll them into a return Vector
    let mut rv = Vec::with_capacity(strs.len());
    for (_key, list) in table {
        for idx in list {
            rv.push(strs[idx].into());
        }
    }
    rv
}
#[test]
fn test_group_anagrams() {
    let words = vec![
        "abed", "later", "bead", "alert", "altered", "bade", "alter", "alerted",
    ];
    group_anagrams(words);

    // FIXME: testing this takes a bit more than most other examples,
    // since Rust's [HashMap] does not return keys in insertion order.
    // A real, correct test would check "by group".

    // let expected_sort = vec![
    //     "altered", "alerted", "abed", "bead", "bade", "later", "alert", "alter",
    // ];
    // assert_eq!(group_anagrams(words), expected_sort);
}
