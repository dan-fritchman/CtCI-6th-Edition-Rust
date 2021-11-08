//!
//! # Insertion:
//!
//! You are given two 32-bit numbers, N and M, and two bit positions, i and j.
//! Write a method to insert M into N such that M starts at bit j and ends at bit i.
//! You can assume that the bits j through i have enough space to fit all of M.
//! That is, if M= 10011, you can assume that there are at least 5 bits between j and i.
//! You would not, for example, have j = 3 and i = 2, because M could not fully fit between bit 3 and bit 2.
//!
//! EXAMPLE
//! Input:  N=10000000000, M=10011, i=2, j=6
//! Output: N=10001001100
//!
//! Hints: # 137, #169, #215
//!

/// Primary Implementation
/// Mask off bits `j` through `i` of `n`, and or-in `m`.
pub fn insertion(n: u32, m: u32, i: u32, j: u32) -> Result<u32, &'static str> {
    if j < i || j > 32 || i > 32 || m > 2_u32.pow(1 + j - i) {
        return Err("Invalid Inputs");
    }

    // Create a bit-mask of bits `j` through `i`
    let j_thru_i = 2_u32.pow(j) - 1 ^ 2_u32.pow(i) - 1;
    // Mask off those bits of `n`
    let mut rv = n & !j_thru_i;
    // And shift in `m`
    rv |= m << i;
    Ok(rv)
}
/// Create a [u32] integer from a binary string.
/// Quick wrapper over [u32::from_str_radix].
pub fn int(s: &str) -> u32 {
    u32::from_str_radix(s, 2).unwrap()
}
#[test]
fn test_insertion() {
    let test_cases = [
        ((int("10000000000"), int("10011"), 2, 6), int("10001001100")),
        ((int("11111111111"), int("10011"), 2, 6), int("11111001111")),
    ];
    for case in test_cases {
        assert_eq!(
            insertion(case.0 .0, case.0 .1, case.0 .2, case.0 .3).unwrap(),
            case.1
        );
    }
}
