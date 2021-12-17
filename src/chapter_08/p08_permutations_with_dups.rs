//!
//! # Permutations with Dups:
//!
//! Write a method to compute all permutations of a string whose characters are not necessarily unique.
//! The list of permutations should not have duplicates.
//!
//! Hints: # 161, #190, #222, #255
//!

use std::collections::{HashMap, HashSet};

/// Primary Implementation
///
/// Create a hashmap of character-counts, recursively pulling one out at each level,
/// tracking the prefix-string to date, and appending it to all permutations of remaining suffix-string.
///
pub fn permutations_with_dups(s: &str) -> HashSet<String> {
    let mut map = char_counts(s);
    let mut set = HashSet::new();
    helper(&mut map, s.len(), "", &mut set);
    set
}
/// Recursive helper (doing most of the real work)
fn helper(
    map: &mut HashMap<char, usize>,
    remchars: usize,
    prefix: &str,
    set: &mut HashSet<String>,
) {
    if remchars == 0 {
        // Base case: completed a permutation
        set.insert(prefix.to_string());
        return;
    }
    // Recurive case. For each entry in the map, add it to the prefix, and recursively create suffixes.
    let keys: Vec<char> = map.keys().cloned().collect();
    for ch in keys {
        // Deduct one from the character-count
        let count = map.get(&ch).unwrap();
        if *count == 1 {
            map.remove(&ch);
        } else {
            *map.get_mut(&ch).unwrap() -= 1;
        }

        // Recursively get subsequent combinations
        helper(map, remchars - 1, &format!("{}{}", prefix, ch), set);

        // And add the character back to the map
        *map.entry(ch).or_insert(0) += 1;
    }
}

/// Take character-counts in string `s`, returning them in a [HashMap].
fn char_counts(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for ch in s.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }
    map
}

#[test]
fn test_permutations_with_dups() {
    let test_cases = [
        ("aaaaaaaaaaaaa", vec!["aaaaaaaaaaaaa"]),
        ("a", vec!["a"]),
        ("ab", vec!["ab", "ba"]),
        ("aab", vec!["aab", "aba", "baa"]),
        ("aabb", vec!["aabb", "bbaa", "baab", "baba", "abab", "abba"]),
        (
            "aabbcc",
            vec![
                "ccabab", "acbbac", "bbacac", "acbcab", "cbbcaa", "bcacab", "babcac", "caabcb",
                "cacabb", "bccbaa", "cababc", "aaccbb", "cbacba", "bcaacb", "cbbaac", "accabb",
                "cabcab", "ccbaba", "cbcaab", "bacabc", "aabccb", "bbcaca", "cbcaba", "caabbc",
                "abbacc", "accbab", "bbccaa", "cacbba", "bcabac", "cabbca", "bcaabc", "cbcbaa",
                "bacbca", "abcbac", "bbaacc", "caacbb", "abbcca", "cbacab", "baabcc", "acabbc",
                "acbbca", "baccab", "ababcc", "abacbc", "abaccb", "abccab", "cbaacb", "cabacb",
                "ccbaab", "bacbac", "abcbca", "abbcac", "babacc", "aacbbc", "baacbc", "bbcaac",
                "cabcba", "bcbcaa", "bcbaca", "abcabc", "aacbcb", "bcabca", "bccaab", "cbaabc",
                "abccba", "ccaabb", "baaccb", "bcacba", "abcacb", "cacbab", "accbba", "acbcba",
                "ccbbaa", "acabcb", "cbabac", "aabbcc", "cbabca", "aabcbc", "babcca", "cabbac",
                "ccabba", "acbacb", "acacbb", "baccba", "bbacca", "bacacb", "bcbaac", "bccaba",
                "cbbaca", "acbabc",
            ],
        ),
    ];
    for case in test_cases {
        // Convert the test-data vector-literal into an [HashSet]
        use std::iter::FromIterator;
        let len = case.1.len();
        let correct = HashSet::from_iter(case.1.into_iter().map(|s| s.to_string()));
        assert_eq!(len, correct.len());

        // Run and check our results
        assert_eq!(permutations_with_dups(case.0), correct);
    }
}
