//!
//! # Coins:
//!
//! Given an infinite number of quarters (25 cents), dimes (10 cents), nickels (5 cents), and pennies (1 cent),
//! write code to calculate the number of ways of representing n cents.
//!
//! Hints: #300, #324, #343, #380, #394
//!

use std::collections::HashMap;

const COIN_SIZES: [isize; 4] = [25, 10, 5, 1];

pub fn coins(amount: isize) -> usize {
    if amount <= 0 {
        return 0;
    }
    let mut seen = HashMap::new();
    helper(amount, 0, &mut seen)
}
pub fn helper(amount: isize, denom: usize, seen: &mut HashMap<(isize, usize), usize>) -> usize {
    if denom >= COIN_SIZES.len() - 1 {
        return 1;
    }
    if seen.contains_key(&(amount, denom)) {
        return *seen.get(&(amount, denom)).unwrap();
    }
    let mut total = 0;
    let size = COIN_SIZES[denom];
    for n in 0..(1 + amount / size) {
        total += helper(amount - n * size, denom + 1, seen);
    }
    seen.insert((amount, denom), total);
    total
}
#[test]
fn test_coins() {
    let test_cases = [
        (1, 1),
        (2, 1),
        (3, 1),
        (4, 1),
        (5, 2),
        (6, 2),
        (7, 2),
        (8, 2),
        (9, 2),
        (10, 4),
        (100, 242),
    ];
    for case in test_cases {
        assert_eq!(coins(case.0), case.1);
    }
}
