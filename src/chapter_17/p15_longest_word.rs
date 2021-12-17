//!
//! # Longest Word:
//!
//! Given a list of words, write a program to find the longest word made of other words in the list.
//!
//! EXAMPLE
//! Input: cat, banana, dog, nana, walk, walker, dogwalker
//! Output: dogwalker
//!
//! Hints: #475, #499, #543, #589
//!

use std::collections::HashMap;

/// Primary Implementation
///
/// For each word, for each character split, recursively check whether
/// each component is composable of other words.
///
pub fn longest_word(mut words: Vec<&str>) -> Option<String> {
    // Sort the input words by length
    words.sort_by_key(|a| a.len());

    // Initialize our cache-map of known-good words
    let mut map = HashMap::new();
    for word in &words {
        map.insert(word.to_string(), true);
    }

    // Now walk over each, checking whether it is composable, and replacing `rv`
    let mut rv = None;
    for word in words.iter().rev() {
        if word.len() >= optlen(&rv) && composable(word, &words, &mut map, true) {
            rv = Some(word.to_string());
        }
    }
    rv
}
fn optlen(opt: &Option<String>) -> usize {
    match opt {
        Some(s) => s.len(),
        None => 0,
    }
}
fn composable(word: &str, words: &[&str], map: &mut HashMap<String, bool>, orig: bool) -> bool {
    if map.contains_key(word) && !orig {
        return *map.get(word).unwrap();
    }
    for idx in 0..word.len() {
        if composable(&word[0..idx], words, map, false)
            && composable(&word[idx..word.len()], words, map, false)
        {
            map.insert(word.to_string(), true);
            return true;
        }
    }
    map.insert(word.to_string(), false);
    false
}

#[test]
fn test_longest_word() {
    let words = vec![
        "cat",
        "banana",
        "dog",
        "nana",
        "walk",
        "walker",
        "dogwalker",
    ];
    let result = longest_word(words);
    assert_eq!(result, Some(String::from("dogwalker")));

    let words = vec!["cat", "banana", "dog"];
    let result = longest_word(words);
    assert_eq!(result, None);

    let words = vec![
        "cat",
        "banana",
        "dog",
        "nana",
        "catbanana",
        "walk",
        "walker",
        "dogwalker",
    ];
    let result = longest_word(words);
    assert_eq!(result, Some(String::from("catbanana")));
}
