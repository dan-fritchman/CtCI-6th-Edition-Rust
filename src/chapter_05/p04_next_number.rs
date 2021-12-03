//!
//! # Next Number:
//!
//! Given a positive integer, print the next smallest and the next largest number that have the same number of 1 bits in their binary representation.
//!
//! Hints: #147, # 175, #242, #312, #339, #358, #375, #390
//!

/// Primary Implementation
///
/// FIXME: this is the brute-force solution.
/// It does take forever given special cases such as zero and `usize::MAX` on a 64-bit machine.
/// McDowell's solutions include several more clever bit-manipulation and arithmetic-based methods.
/// Implementing these has, thus far, not bubble to the top of this effort's priorities.
///
pub fn next_number(num: usize) -> (Option<usize>, Option<usize>) {
    // Count the number of one-bits in `num`
    let ones = count(num);

    // Cycle down from `num`, searching for another number with the same `count`
    let mut low_rv = None;
    {
        let mut low = num;
        while low > 0 {
            low -= 1;
            if count(low) == ones {
                low_rv = Some(low);
                break;
            }
        }
    }

    // Cycle up from `num`, searching for another number with the same `count`
    let mut high_rv = None;
    {
        let mut high = num;
        while high < usize::MAX {
            high += 1;
            if count(high) == ones {
                high_rv = Some(high);
                break;
            }
        }
    }
    (low_rv, high_rv)
}

/// Count the number of one-bits in `num`
fn count(mut num: usize) -> usize {
    let mut exp = 1;
    let mut count = 0;
    while num > 0 {
        if num % 2_usize.pow(exp) > 0 {
            num -= 2_usize.pow(exp - 1);
            count += 1;
        }
        exp += 1;
    }
    count
}

#[test]
fn test_next_number() {
    let test_cases = [
        (1, (None, Some(2))),
        (2, (Some(1), Some(4))),
        (3, (None, Some(5))),
        (4, (Some(2), Some(8))),
        (5, (Some(3), Some(6))),
        // FIXME: Special cases
        // (0, (None, None)),
        // (usize::MAX, (None, None)),
    ];
    for case in test_cases {
        assert_eq!(next_number(case.0), case.1);
    }
}
