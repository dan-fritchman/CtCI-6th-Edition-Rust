//!
//! # Palindrome:
//!
//! Implement a function to check if a linked list is a palindrome.
//!
//! Hints: #5, #13, #29, #61, #101
//!

// Local Imports
use super::utils::List;

///
/// Primary Implementation
///
/// Test whether [List] `list` is a palindrome.
/// Since [List] is doubly-linked, this just requires two pointers walking from `head` and `tail`,
/// comparing values along the way.
/// Returns `true` if (a) the `head` and `tail` chases end without encountering a difference.
/// Panics if the list (somehow) has different numbers of elements when traversing forward and backward.
///
/// Note this does about 2x as many comparisons as it would need to in theory,
/// as it could stop at the list's "midpoint" instead of traversing all the way across.
///
pub fn is_palindrome<T: PartialEq >(list: &List<T>) -> bool {
    // Initialize forward and backward pointers
    let mut head = list.head;
    let mut tail = list.tail;
    // And iterate over a pair at a time
    loop {
        match (head, tail) {
            (Some(h), Some(t)) => {
                if list[h].data != list[t].data {
                    return false;
                }
                head = list[h].next;
                tail = list[t].prev;
            }
            (None, None) => return true, // Done! It's a palindrome!
            _ => unreachable!(),         // Panics if the two traversals end at different times
        }
    }
}

#[test]
fn test_palindrome() {
    let test_cases = [
        (vec![1, 2, 3, 4, 3, 2, 1], true),
        (vec![1], true),
        (vec![], true),
        (vec![1, 2, 3, 4, 5], false),
        (vec![1, 2], false),
    ];
    for case in test_cases {
        let list = List::from(case.0.as_slice());
        assert_eq!(is_palindrome(&list), case.1);
    }

    // Now do some character-valued cases
    let test_cases = [(vec!['a', 'a'], true), (vec!['a', 'b', 'a'], true)];
    for case in test_cases {
        let list = List::from(case.0.as_slice());
        assert_eq!(is_palindrome(&list), case.1);
    }
}
