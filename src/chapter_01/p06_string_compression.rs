//!
//! # String Compression:
//!
//! Implement a method to perform basic string compression using the counts of repeated characters.
//! For example, the string aabcccccaaa would become a2b1c5a3.
//! If the "compressed" string would not become smaller than the original string, your method should return the original string.
//! You can assume the string has only uppercase and lowercase letters (a - z).
//!
//! Hints: #92, # 110
//!

pub fn compress_str(s: &str) -> String {
    let mut compressed = String::new();
    if s.is_empty() {
        return compressed;
    }
    let mut chars = s.chars();

    // Pop off the first character as our loop prelude
    let mut prev = chars.next().unwrap();
    if !prev.is_alphabetic() {
        panic!("BUT BUT BUT you said they'd all be letters!");
    }
    prev = prev.to_lowercase().next().unwrap();

    // Initialize a count of consecutive identical characters
    let mut count = 1;

    // And iterate over the rest of the string
    for c in chars {
        if !c.is_alphabetic() {
            panic!("BUT BUT BUT you said they'd all be letters!");
        }
        if c.to_lowercase().next().unwrap() == prev {
            count += 1;
        } else {
            compressed.push(prev);
            compressed.push_str(&count.to_string());
            prev = c;
            count = 1;
        }
        if compressed.len() >= s.len() {
            return s.to_string();
        }
    }
    // Now we're at the end of the string, so we need to push the last character(s)
    compressed.push(prev);
    compressed.push_str(&count.to_string());
    if compressed.len() >= s.len() {
        return s.to_string();
    }
    // Finished, return the compressed string
    compressed
}

#[test]
fn test_compress_str() {
    let test_cases = [
        ("aabcccccaaa", "a2b1c5a3"),
        ("abcdef", "abcdef"),
        ("aabb", "aabb"),
        ("aaa", "a3"),
        ("a", "a"),
        ("", ""),
    ];
    for case in test_cases {
        assert_eq!(compress_str(&case.0), case.1);
    }
}
