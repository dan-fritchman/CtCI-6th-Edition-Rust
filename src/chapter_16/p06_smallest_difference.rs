//!
//! # Smallest Difference:
//!
//! Given two arrays of integers, compute the pair of values (one value in each array)
//! with the smallest (non-negative) difference. Return the difference.
//!
//! EXAMPLE
//! Input: {1, 3, 15, 11, 2}, {23, 127, 235, 19, 8}
//! Output: 3. That is, the pair (11 , 8).
//!
//! Hints: #632, #670, #679
//!

/// Primary Implementation
///
/// The real goal here is to avoid doing something that takes N^2 (or A*B) time.
/// So, we sort both arrays, then use a "pointer chasing" two-index pass across them.
/// Should take on the complexity of the sort.
/// (Which we don't implement here, but should be N*logN.)
///
pub fn smallest_difference(a: &mut [isize], b: &mut [isize]) -> Option<SmallestDiff> {
    if a.is_empty() || b.is_empty() {
        // Convey the problematic empty-vecs case.
        return None;
    }
    // Sort both arrays
    a.sort_unstable();
    b.sort_unstable();

    // And two-pointer walk through them
    let mut aptr = 0;
    let mut bptr = 0;
    let mut smallest = (a[0] - b[0]).abs();
    let mut amin = 0;
    let mut bmin = 0;
    loop {
        let diff = (a[aptr] - b[bptr]).abs();
        if diff < smallest {
            smallest = diff;
            amin = a[aptr];
            bmin = b[bptr];
        }
        // Now update to the next index, or break out
        if aptr == a.len() - 1 && bptr == b.len() - 1 {
            break; // Done
        } else if aptr == a.len() - 1 {
            bptr += 1; // `a` is done, `b` has more elements
        } else if bptr == b.len() - 1 {
            aptr += 1; // `b` is done, `a` has more elements
        } else if a[aptr] > b[bptr] {
            bptr += 1; // Increment the smaller of `a` and `b`
        } else {
            aptr += 1;
        }
    }
    Some(SmallestDiff::from((smallest, amin, bmin)))
}

/// Result Struct for `smallest_diff`
#[derive(Debug, PartialEq, Eq)]
pub struct SmallestDiff {
    pub difference: isize,
    pub a_elem: isize,
    pub b_elem: isize,
}
impl From<(isize, isize, isize)> for SmallestDiff {
    fn from(tup: (isize, isize, isize)) -> Self {
        Self {
            difference: tup.0,
            a_elem: tup.1,
            b_elem: tup.2,
        }
    }
}

#[test]
fn test_smallest_difference() {
    let test_cases = [
        (vec![1, 3, 15, 11, 2], vec![23, 127, 235, 19, 8], (3, 11, 8)),
        (
            vec![2, 11, 15, 1],
            vec![12, 4, 235, 19, 127, 23],
            (1, 11, 12),
        ),
        (
            vec![32, 1, 5, -31],
            vec![98, 7, 32, 43, 72, 86],
            (0, 32, 32),
        ),
    ];
    for case in test_cases {
        let mut a = case.0;
        let mut b = case.1;
        let exp = Some(SmallestDiff::from(case.2));
        assert_eq!(smallest_difference(&mut a, &mut b), exp);
    }
}
