//!
//! # Trie Word-Trees
//!

use std::collections::HashMap;

/// # Trie
///
/// Tree-based collection of words, with character-based [Node]s.
/// Stores children in a [HashMap], in which keys are equal to their `value` fields.
///
#[derive(Debug, Default)]
pub struct Trie {
    pub root_nodes: HashMap<char, Node>,
}
impl Trie {
    /// Create a [Trie] from a list of `words`.
    pub fn create(words: &[&str]) -> Self {
        let mut trie = Self::default();
        for word in words {
            trie.add_word(word);
        }
        trie
    }
    /// Add string `word`, a character-node at a time. Re-use any existing char-nodes.
    pub fn add_word(&mut self, word: &str) {
        let mut children = &mut self.root_nodes;
        for c in word.chars() {
            let node = children.entry(c).or_insert(Node::new(c));
            children = &mut node.children;
        }
        children.insert(ENDCHAR, Node::new(ENDCHAR));
    }
}

/// [Trie] character-[Node].
/// Stores children in a [HashMap], in which keys are equal to their `value` fields.
#[derive(Debug, Default)]
pub struct Node {
    pub value: char,
    pub children: HashMap<char, Node>,
}
impl Node {
    pub fn new(value: char) -> Self {
        Self {
            value,
            ..Default::default()
        }
    }
}

/// Cardinal character-value used to represent "end of word".
pub const ENDCHAR: char = '*';
