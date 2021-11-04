//!
//! # Sum Lists:
//!
//! You have two numbers represented by a linked list, where each node contains a single digit.
//! The digits are stored in reverse order, such that the 1's digit is at the head of the list.
//! Write a function that adds the two numbers and returns the sum as a linked list.
//!
//! EXAMPLE
//! Input: (7 -> 1 -> 6) + (5 -> 9 -> 2).
//! That is, 617 + 295.
//! Output: 2 -> 1 -> 9.
//! That is, 912.
//!
//! FOLLOW UP
//! Suppose the digits are stored in forward order. Repeat the above problem.
//!
//! EXAMPLE
//! Input: (6 -> 1 -> 7) + (2 -> 9 -> 5).
//! That is, 617 + 295.
//! Output: 9 -> 1 -> 2.
//! That is, 912.
//!
//! Hints: #7, #30, #71, #95, #109
//!

// Local Imports
use super::utils::List;

/// Integer-Valued Linked List Representing Decimal Numbers
/// Starting from ones place in head entry
pub struct NumList {
    /// Internal linked list
    list: List<isize>,
}
/// Create from an [isize] integer
impl From<isize> for NumList {
    fn from(mut i: isize) -> Self {
        let mut list = List::new();
        while i > 0 {
            list.push(i % 10);
            i /= 10;
        }
        Self { list }
    }
}
/// Convert to an [isize] integer
impl From<NumList> for isize {
    fn from(numls: NumList) -> isize {
        let mut rv = 0;
        let mut mult = 1;

        let mut ptr = numls.list.head;
        while let Some(idx) = ptr {
            ptr = numls.list[idx].next;
            rv += numls.list[idx].data * mult;
            mult *= 10;
        }
        rv
    }
}
pub fn sum_lists(a: &NumList, b: &NumList) -> NumList {
    // Keep a running-tally return value, and scale multiplier
    let mut rv: isize = 0;
    let mut mult: isize = 1;

    // And initialize pointers into each list
    let mut aptr = a.list.head;
    let mut bptr = b.list.head;

    while aptr.is_some() || bptr.is_some() {
        // Grab the values from each
        let aval = match aptr {
            Some(idx) => a.list[idx].data,
            None => 0,
        };
        let bval = match bptr {
            Some(idx) => b.list[idx].data,
            None => 0,
        };

        // Add in its contribution
        rv += (aval + bval) * mult;

        // And bump the scale multiplier
        mult *= 10;

        // Advance each pointer, if not at the end of its list
        if let Some(idx) = aptr {
            aptr = a.list[idx].next;
        }
        if let Some(idx) = bptr {
            bptr = b.list[idx].next;
        }
    }
    NumList::from(rv)
}
#[test]
fn test_sum_lists() {
    let test_cases = [
        (617, 295, 912),
        (0, 0, 0),
        (123, 123, 246),
        (123, 1, 124),
        (1, 123, 124),
    ];
    for case in test_cases {
        let a = NumList::from(case.0);
        let b = NumList::from(case.1);
        let sum: isize = sum_lists(&a, &b).into();
        assert_eq!(sum, case.2);
    }
}
