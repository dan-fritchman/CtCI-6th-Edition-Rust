//!
//! # Parens:
//!
//! Implement an algorithm to print all valid (e.g., properly opened and closed) combinations of n pairs of parentheses.
//!
//! EXAMPLE
//!
//! Input: 3
//! Output: «()))J «)(», «» ()J () ( (», ()()()
//!
//! Hints: # 138, #174, #187, #209, #243, #265, #295
//!

pub fn parens(_n: usize) -> String {
    todo!()
}

#[ignore]
#[test]
fn test_parens() {
    let test_cases = [
        (0, vec![""]),
        (1, vec!["()"]),
        (2, vec![["()()", "(())"]]),
        (3, vec![["((()))", "(()())", "(())()", "()(())", "()()()"]]),
    ];
    for case in test_cases {
        assert_eq!(parens(case.0), case.1);
    }
}
