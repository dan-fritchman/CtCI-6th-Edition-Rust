//!
//! # Multi Search:
//!
//! Given a string b and an array of smaller strings T, design a method to search b for
//! each small string in T.
//!
//! Hints: #480, #582, #617, #743
//!

use std::collections::HashMap;
// Local Imports
use crate::trie::{Node, Trie, ENDCHAR};

/// Primary Implementation
///
/// Create a search [Trie] of the small-words list,
/// then search `big_word` starting from each character for matches in that trie.
/// 
pub fn multi_search<'big, 'sm>(
    big_word: &'big str,
    small_words: &[&'sm str],
) -> HashMap<String, Vec<usize>> {
    // Create the search [Trie]
    let trie = Trie::create(small_words);

    // Iterate over each sub-string of `big_word`, checking for matches in the trie.
    let mut rv = HashMap::new();
    for idx in 0..big_word.len() {
        let hits = search(&trie, &big_word[idx..big_word.len()]);
        for s in hits {
            rv.entry(s).or_insert(Vec::new()).push(idx);
        }
    }
    rv
}

/// Search sub-string `subword` for hits in the trie.
/// Returns a list of entries in the trie that match from the *beginning* of subword.
fn search(trie: &Trie, subword: &str) -> Vec<String> {
    let mut chars = subword.chars().enumerate();
    let mut rv = Vec::new();
    let mut children: &HashMap<char, Node> = &trie.root_nodes;
    loop {
        // Read the next character (and index), or return if empty.
        let (idx, c) = match chars.next() {
            Some((i, c)) => (i, c),
            None => return rv,
        };
        // And check for the character in our `children`-keys, or return if missing.
        let node = match children.get(&c) {
            Some(n) => n,
            None => return rv,
        };
        // If this is the end of a word, add it to our hits-list.
        if node.children.contains_key(&ENDCHAR) {
            rv.push(subword[..idx + 1].to_string());
        }
        // And update the `children` pointer.
        children = &node.children;
    }
}

#[test]
fn test_multi_search() {
    let small_words = vec!["i", "is", "pp", "ms"];
    let mut expected = HashMap::new();
    expected.insert(String::from("i"), vec![1, 4, 7, 10]);
    expected.insert(String::from("is"), vec![1, 4]);
    expected.insert(String::from("pp"), vec![8]);
    assert_eq!(multi_search("mississippi", &small_words), expected);
}
