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

pub fn word_transformer(_src: &str, _des: &str, _dict: &[&str]) -> Vec<String> {
    todo!()
}

#[ignore]
#[test]
fn test_word_transformer() {
    let source = "bit";
    let target = "dog";
    let dictionary = vec!["but", "put", "big", "pot", "pog", "dog", "lot"];
    let expected = vec!["bit", "but", "put", "pot", "pog", "dog"];
    assert_eq!(word_transformer(source, target, &dictionary), expected);

    let source = "damp";
    let target = "like";
    let dictionary = vec!["damp", "lime", "limp", "lamp", "like"];
    let expected = vec!["damp", "lamp", "limp", "lime", "like"];
    assert_eq!(word_transformer(source, target, &dictionary), expected);
}
