//!
//! # Recursive Multiply:
//!
//! Write a recursive function to multiply two positive integers without using the * operator.
//! You can use addition, subtraction, and bit shifting, but you should minimize the number of those operations.
//!
//! Hints: # 166, #203, #227, #234, #246, #280
//!

pub fn recursive_multiply(_a: usize, _b: usize) -> usize {
    todo!()
}

#[ignore]
#[test]
fn test_recursive_multiply() {
    let test_cases = [(5, 6), (28, 89), (1234, 245334)];
    for case in test_cases {
        assert_eq!(recursive_multiply(case.0, case.1), case.0 * case.1);
    }
}
