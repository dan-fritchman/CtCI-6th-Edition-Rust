//!
//! # Add Without Plus:
//!
//! Write a function that adds two numbers.
//! You should not use + or any arithmetic operators.
//!
//! Hints: #467, #544, #601, #628, #642, #664, #692, #7 12, #724
//!

/// Primary Implementation
///
/// Run a ripple-carry adder on `a` and `b`'s bits!
///
pub fn add_without_plus(a: i32, b: i32) -> i32 {
    let mut rv = 0;
    let mut carry = 0;
    for bit in 0..32 {
        // Mask out the relevant bit of `a` and `b`
        let bita = (a >> bit) & 1;
        let bitb = (b >> bit) & 1;
        // Perform the full-adder operations
        let sum = carry ^ bita ^ bitb;
        carry = (bita & bitb) | (bita & carry) | (bitb & carry);
        // And or-in the summand
        rv |= sum << bit;
    }
    rv
}
#[test]
fn test_add_without_plus() {
    let test_cases = [
        (0, 0, 0),
        (1, 2, 3),
        (-1, 1, 0),
        (4, 5, 9),
        (55, 11, 66),
        (1023, -1024, -1),
        (2_i32.pow(16), 2_i32.pow(16), 2_i32.pow(17)),
        // Test some wrap-around cases
        (i32::MAX, 1, -i32::MAX - 1),
        (i32::MAX, i32::MAX, -2),
    ];
    for case in test_cases {
        assert_eq!(add_without_plus(case.0, case.1), case.2);
    }
}
