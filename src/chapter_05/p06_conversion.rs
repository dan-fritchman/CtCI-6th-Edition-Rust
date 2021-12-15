//!
//! # Conversion:
//!
//! Write a function to determine the number of bits you would need to flip to convert integer A to integer B.
//!
//! EXAMPLE
//!
//! Input: 29 (or: 11101), 15 (or: 01111)
//! Output: 2
//!
//! Hints: #336, #369
//!

/// Primary Implementation
///
/// Count the number of one-bits in `a XOR b`.
/// Efficiently performs one-counting by repeatedly negating the most-significant one-bit,
/// but ANDing `x` with `x-1`.
pub fn conversion(a: usize, b: usize) -> usize {
    let mut x = a ^ b;
    let mut count = 0;
    while x > 0 {
        x = x & (x - 1);
        count += 1;
    }
    count
}

#[test]
fn test_conversion() {
    let test_cases = [
        (0, 0, 0),
        (0, 1, 1),
        (0, 2, 1),
        (1, 2, 2),
        (29, 15, 2),
        (15, 29, 2),
    ];
    for case in test_cases {
        assert_eq!(conversion(case.0, case.1), case.2);
    }
}
