//!
//! # Recursive Multiply:
//!
//! Write a recursive function to multiply two positive integers without using the * operator.
//! You can use addition, subtraction, and bit shifting, but you should minimize the number of those operations.
//!
//! Hints: # 166, #203, #227, #234, #246, #280
//!

/// Primary Implementation
///
/// Recursively take binary-weighted partial-products and sum their total.
///
pub fn recursive_multiply(a: usize, b: usize) -> usize {
    let min = a.min(b);
    let max = a.max(b);
    helper(min, max)
}
fn helper(min: usize, max: usize) -> usize {
    match min {
        0 => 0,   // Base case: 0 * max = 0
        1 => max, // Base case: 1 * max = max
        _ => {
            // Recursively get a sub-product, and double it
            let half = helper(min / 2, max);
            let mut result = half + half;
            // And if `min` was odd, add another `max` to the result.
            if min % 2 == 1 {
                result += max;
            }
            result
        }
    }
}

#[test]
fn test_recursive_multiply() {
    let test_cases = [
        (0, 0),
        (0, 1),
        (1, 1),
        (5, 6),
        (28, 89),
        (32, 64),
        (127, 255),
        (1234, 245334),
    ];
    for case in test_cases {
        assert_eq!(recursive_multiply(case.0, case.1), case.0 * case.1);
    }
}
