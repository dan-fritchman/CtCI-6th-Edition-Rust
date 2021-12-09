//!
//! # Search in Rotated Array:
//!
//! Given a sorted array of n integers that has been rotated an unknown number of times,
//! write code to find an element in the array.
//! You may assume that the array was originally sorted in increasing order.
//!
//! EXAMPLE
//! Input: find 5 in {15, 16, 19, 20, 25, 1, 3, 4, 5, 7, 10, 14}
//! Output: 8 (the index of 5 in the array)
//!
//! Hints: #298, #310
//!

/// Primary Implementation
///
/// A modified binary search, first identifying which of the left and right halves
/// are in-order, by comparing the mid-point and end-points.
///
pub fn search_in_rotated_array(arr: &[isize], targ: isize) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }
    let mid = arr.len() / 2;
    if arr[mid] == targ {
        return Some(mid);
    }
    if arr[0] < arr[mid] {
        // Left is ordered
        if arr[0] <= targ && targ <= arr[mid] {
            // Search the left
            return search_in_rotated_array(&arr[0..mid], targ);
        }
        // Search the right. Add `mid` to the index if it hits.
        return search_in_rotated_array(&arr[mid..arr.len()], targ).map(|idx| idx + mid);
    }
    if arr[mid] < arr[0] {
        // Right is ordered
        if arr[mid] <= targ && targ <= *arr.last().unwrap() {
            // Search the right, and add `mid`
            return search_in_rotated_array(&arr[mid..arr.len()], targ).map(|idx| idx + mid);
        }
        // Search the left
        return search_in_rotated_array(&arr[0..mid], targ);
    }
    if arr[mid] == arr[0] {
        if *arr.last().unwrap() != arr[mid] {
            // Right is distinct, search it
            return search_in_rotated_array(&arr[mid..arr.len()], targ).map(|idx| idx + mid);
        }
        // Left == Right == Mid. Need to search both halves, starting with the left.
        match search_in_rotated_array(&arr[0..mid], targ) {
            Some(res) => return Some(res),
            None => (),
        };
        return search_in_rotated_array(&arr[mid..arr.len()], targ).map(|idx| idx + mid);
    }
    None // None of these cases hit - `target` was not found.
}

#[test]
fn test_search_in_rotated_array() {
    assert_eq!(
        search_in_rotated_array(&[15, 16, 19, 20, 25, 1, 3, 4, 5, 7, 10, 14], 5),
        Some(8)
    );
    assert_eq!(
        search_in_rotated_array(&[2, 2, 2, 3, 4, 2, 2, 2, 2, 2, 2, 2, 2, 2], 3),
        Some(3)
    );
}
