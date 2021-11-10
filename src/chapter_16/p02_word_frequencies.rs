//!
//! # Word Frequencies:
//!
//! Design a method to find the frequency of occurrences of any given word in a book.
//! What if we were running this algorithm multiple times?
//!
//! Hints: #489, #536
//!

// The "multiple times" hint implies we should index "offline", then re-run against that index,
// similar to how a web-search engine might work.
// Here the `index` function will perform that should-be-offline work,
// but since we've only got a since entry-point, it does it inline.

use std::collections::HashMap;

/// Primary Implementation
pub fn word_frequencies(book: &'static str, word: &str) -> usize {
    // Do the "offline" indexing
    let table = index(book);
    // And retrieve the count from the index table
    *table.get(&word.to_lowercase()).unwrap_or(&0)
}
/// The "web crawler", or at least our index-er of word-counts.
fn index(book: &'static str) -> HashMap<String, usize> {
    let mut table = HashMap::new();
    for word in book.split_whitespace() {
        // Convert our word to lower-case, and remove any punctuation
        let word = word.replace(&['(', ')', ',', '\"', '.', ';', ':', '\''][..], "");
        let word = word.to_lowercase();
        // Increment or create its count in our table
        *table.entry(word).or_insert(0) += 1;
    }
    table
}

#[test]
fn test_word_frequencies() {
    let book = "Once upon a time there was this book.
            This is a sentence. This is a much longer sentence.
            This book is terribly short. But you get the idea.
            You should see the word this 6 times in this example text.
            ";
    assert_eq!(word_frequencies(book, "book"), 2);
    assert_eq!(word_frequencies(book, "this"), 6);
}
