//!
//! # Pairwise Swap:
//!
//! Write a program to swap odd and even bits in an integer with as few instructions as possible
//! (e.g., bit 13 and bit 1 are swapped, bit 2 and bit 3 are swapped, and so on).
//!
//! Hints: #145, #248, #328, #355
//!

pub fn pairwise_swap(_n: usize) -> usize {
    todo!()
}

#[ignore]
#[test]
fn test_pairwise_swap() {
    let test_cases = [(123, 183), (781, 782), (278, 553)];
    for case in test_cases {
        assert_eq!(pairwise_swap(case.0), case.1);
    }
}
