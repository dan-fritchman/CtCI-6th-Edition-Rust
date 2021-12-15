//!
//! # Pairwise Swap:
//!
//! Write a program to swap odd and even bits in an integer with as few instructions as possible
//! (e.g., bit 13 and bit 1 are swapped, bit 2 and bit 3 are swapped, and so on).
//!
//! Hints: #145, #248, #328, #355
//!

/// Primary Implementation
///
/// Filter odd and even bits, shift each by one in opposite directions,
/// and or the two back together.
///
pub fn pairwise_swap(n: u32) -> u32 {
    let evens = n & 0x5555_5555;
    let odds = n & 0xAAAA_AAAA;
    // Note for Rusts's unsigned integers, the shift operators perform *logical* shifts.
    (odds >> 1) | (evens << 1)
}

#[test]
fn test_pairwise_swap() {
    let test_cases = [(123, 183), (781, 782), (278, 553)];
    for case in test_cases {
        assert_eq!(pairwise_swap(case.0), case.1);
    }
}
