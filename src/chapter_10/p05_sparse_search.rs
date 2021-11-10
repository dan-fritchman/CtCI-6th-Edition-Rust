//!
//! # Sparse Search: Given a sorted array of strings that is interspersed with empty strings, write a method to find the location of a given string.
//!
//! EXAMPLE
//!
//! Input: "ball", {"at", "", "", "",
//! Output: 4
//!
//! Hints: #256
//!

// String ordering is a place where Rust is, of course, weird.
//
// From https://doc.rust-lang.org/std/cmp/trait.PartialOrd.html#impl-PartialOrd%3Cstr%3E -
//
// ```text
// Strings are compared lexicographically by their byte values.
// This compares Unicode code points based on their positions in the code charts.
// This is not necessarily the same as “alphabetical” order, which varies by language and locale.
// Comparing strings according to culturally-accepted standards requires locale-specific data that is outside the scope of the str type.
// ```
//
// For the English-language strings we'll probably only ever test, this probably works well enough.
//

/// Primary Implementation
///
/// Do binary search, but drop into two-dimensional linear search when he hit empty strings.
///
pub fn sparse_search(arr: &[&str], target: &str) -> Option<usize> {
    // Kick off our recursive helper
    helper(arr, target, 0)
}
/// Recursive helper
/// Chops up slices of the initial `arr` for recursive calls,
/// and must track the start-index `start` so we can return indices into the original array.
pub fn helper(arr: &[&str], target: &str, start: usize) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let index = arr.len() / 2;
    if arr[index] == target {
        Some(index + start)
    } else if arr[index].is_empty() {
        // Empty string case. Start linear-searching, in both directions,
        // for the next non-empty entry in either direction.
        let (mut left, mut right) = ((index - 1) as isize, (index + 1) as isize);
        while left >= 0 && arr[left as usize].is_empty() {
            left -= 1;
        }
        while right < arr.len() as isize && arr[right as usize].is_empty() {
            right += 1;
        }
        // Quickly convert them to [Option<usize>]
        let left = if left < 0 { None } else { Some(left as usize) };
        let right = if right >= arr.len() as isize {
            None
        } else {
            Some(right as usize)
        };

        // And dispatch based on which were found
        match (left, right) {
            (None, None) => None, // No other non-empty elements
            (Some(lf), None) => helper(&arr[0..lf + 1], target, start),
            (None, Some(rt)) => helper(&arr[rt..arr.len()], target, start + rt),
            (Some(lf), Some(rt)) => {
                // Now there are three possible cases:
                // `target` is either 
                // (a) <= than `arr[left]`, 
                // (b) >= arr[right], or 
                // (c) in-between the two.
                // In the latter case we fail, since we've established there are no non-empty elements between the two.
                if target <= arr[lf] {
                    helper(&arr[0..lf + 1], target, start)
                } else if target >= arr[rt] {
                    helper(&arr[rt..arr.len()], target, start + rt)
                } else {
                    None
                }
            }
        }
    } else if arr[index] > target {
        helper(&arr[0..index - 1], target, start)
    } else {
        helper(&arr[index + 1..arr.len()], target, start + index + 1)
    }
}

#[test]
fn test_sparse_search() {
    let test_cases = [
        (
            vec![
                "a", "", "", "b", "", "c", "", "", "d", "", "", "", "", "e", "",
            ],
            "d",
            Some(8),
        ),
        (
            vec![
                "a", "", "", "b", "", "c", "", "", "d", "", "", "", "", "e", "",
            ],
            "f",
            None,
        ),
        (
            vec![
                "a", "", "", "b", "", "c", "", "", "d", "", "", "", "", "e", "",
            ],
            "a",
            Some(0),
        ),
    ];
    for case in test_cases {
        assert_eq!(sparse_search(&case.0, case.1), case.2)
    }
}
