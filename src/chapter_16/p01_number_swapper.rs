//!
//! # Number Swapper:
//!
//! Write a function to swap a number in place (that is, without temporary vari- ables) .
//!
//! Hints: #492, #716, #737
//!

use std::ops::BitXor;

/// Primary Implementation
///
/// Uses the famous "xor trick", for any generic type which supports [BitXor].
/// Swapped variables `a` and `b` are supplied as mutable references to any such type.
///
pub fn swap<T: Copy + BitXor<Output = T>>(a: &mut T, b: &mut T) {
    *a = *a ^ *b;
    *b = *a ^ *b;
    *a = *a ^ *b;
}

#[test]
fn test_swap() {
    let test_cases = [[1, 2], [10, 3], [4, 4], [1, 0], [7, -4], [-7, -4]];
    for case in test_cases {
        let (mut a, mut b) = (case[0].clone(), case[1].clone());
        swap(&mut a, &mut b);
        assert_eq!(a, case[1]);
        assert_eq!(b, case[0]);
    }
}
