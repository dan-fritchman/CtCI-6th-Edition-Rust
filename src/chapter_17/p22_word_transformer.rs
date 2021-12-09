//!
//! # Word Transformer:
//!
//! Given two words of equal length that are in a dictionary,
//! write a method to transform one word into another word by changing only one letter at a time.
//! The new word you get in each step must be in the dictionary.
//!
//! EXAMPLE
//! Input: DAMP, LIKE
//! Output: DAMP -> LAMP -> LIMP -> LIME -> LIKE
//!
//! Hints: #506, #535, #556, #580, #598, #618, #738
//!

use std::collections::{HashMap, HashSet};

/// Primary Implementation
///
/// First creates a [HashMap] of "wildcard equivalent" words, which maps from strings with a single wild-card character
/// to a [HashSet] of matching words in our corpus.
/// E.g.  "b_g" => {"bug", "big", "bog"}, ...
///
/// With this in hand, breadth-first traverse from both `src` and `dest` words,
/// examining each single-wild-card-different word adjacent to each node.
///
pub fn word_transformer(src: &str, dest: &str, dict: &[&str]) -> Option<Vec<String>> {
    // Build up a wild-carded-words dictionary
    let mut wildcards: HashMap<String, HashSet<&str>> = HashMap::new();
    for word in dict {
        for idx in 0..word.len() {
            wildcards
                .entry(wildcard(word, idx))
                .or_insert(HashSet::new())
                .insert(word);
        }
    }

    // Create two [PathGraph]s, emanating from the source and destination words
    let mut from_dest = PathGraph::start(dest);
    let mut from_src = PathGraph::start(src);

    // Take turns attempting to breadth-first advance from the source and destination sides
    while !from_dest.workq.is_empty() || !from_src.workq.is_empty() {
        if let Some(hit) = from_src.search_layer(&from_dest.done, &wildcards) {
            return Some(unwind(hit, &from_src, &from_dest));
        }
        if let Some(hit) = from_dest.search_layer(&from_src.done, &wildcards) {
            return Some(unwind(hit, &from_src, &from_dest));
        }
    }
    None // Both queues empty and no hit found. Return `None`.
}

/// Unwind the paths from src -> `word` -> dest into a vector
fn unwind(word: &str, from_src: &PathGraph, from_dest: &PathGraph) -> Vec<String> {
    // Unwind the part from `word` to `dest`
    let mut dest_vec = Vec::new();
    let mut next_word = Some(word);
    while let Some(w) = next_word {
        dest_vec.push(w.to_string());
        let node = from_dest.nodes.get(w).unwrap();
        next_word = node.prev;
    }
    // Now the part from `word` to `src`, noting this will be in reverse-order
    let mut src_vec = Vec::new();
    let mut next_word = Some(word);
    while let Some(w) = next_word {
        src_vec.push(w.to_string());
        let node = from_src.nodes.get(w).unwrap();
        next_word = node.prev;
    }
    // Pop the (initially first) entry, `word`, and reverse it.
    src_vec.remove(0);
    src_vec.reverse();

    // And append the two together
    src_vec.extend(dest_vec);
    src_vec
}

/// Create a wild-card word replacement for `word`, placing the wild-card at character `idx`.
pub fn wildcard(word: &str, idx: usize) -> String {
    let mut wildcard = word[0..idx].to_string();
    wildcard.push('_'); // Insert the wildcard character
    wildcard.push_str(&word[idx + 1..word.len()]);
    wildcard
}

pub struct PathGraph<'d> {
    nodes: HashMap<&'d str, PathNode<'d>>,
    workq: HashSet<&'d str>,
    done: HashSet<&'d str>,
}
impl<'d> PathGraph<'d> {
    /// Start a new [Path] with word `word`
    pub fn start(word: &'d str) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(word, PathNode { prev: None });
        let mut workq = HashSet::new();
        workq.insert(word);
        Self {
            nodes,
            workq,
            done: HashSet::new(),
        }
    }
    /// Search a layer of the graph, working through its current `workq`.
    /// Returns a `Some(word)` if an element in the work-queue is also in `hits`.
    /// Otherwise returns `None`, and prepares a new `workq` for a further layer.
    fn search_layer(
        &mut self,
        hits: &HashSet<&'d str>,
        wildcards: &'d HashMap<String, HashSet<&'d str>>,
    ) -> Option<&'d str> {
        // Create the next-layer work-queue, collected as we examine this layer's contents.
        let mut nextq = HashSet::new();
        for word in self.workq.iter() {
            if hits.contains(word) {
                return Some(word); // A hit! We're done, save for path unwinding.
            }
            self.done.insert(word);

            // Check each character-wild-carded revision to `word`
            for idx in 0..word.len() {
                let wildset = wildcards.get(&wildcard(word, idx)).unwrap();
                for neighbor in wildset {
                    if !self.done.contains(neighbor) {
                        nextq.insert(*neighbor);
                        self.nodes.insert(neighbor, PathNode { prev: Some(word) });
                    }
                }
            }
        }
        // Not found. update for next time, and return `None`
        self.workq = nextq;
        None
    }
}

/// Node-Type in a [PathGraph].
/// Just stores a previous-node pointer; word-values are stored as the keys in the `PathGraph.nodes` hashmap.
pub struct PathNode<'d> {
    prev: Option<&'d str>,
}

#[test]
fn test_word_transformer() {
    let source = "bit";
    let target = "dog";
    let dictionary = vec!["bit", "but", "put", "big", "pot", "pog", "dog", "lot"];
    let expected: Option<Vec<String>> = Some(
        vec!["bit", "but", "put", "pot", "pog", "dog"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    );
    assert_eq!(word_transformer(source, target, &dictionary), expected);

    let source = "damp";
    let target = "like";
    let dictionary = vec!["damp", "lime", "limp", "lamp", "like"];
    let expected: Option<Vec<String>> = Some(
        vec!["damp", "lamp", "limp", "lime", "like"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    );
    assert_eq!(word_transformer(source, target, &dictionary), expected);
}
