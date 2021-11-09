//!
//! # Triple Step:
//!
//! A child is running up a staircase with n steps and can hop either 1 step, 2 steps, or 3 steps at a time.
//! Implement a method to count how many possible ways the child can run up the stairs.
//!
//! Hints: # 152, # 178, #217, #237, #262, #359
//!

use std::collections::HashMap;

/// Primary Implementation
///
/// Recursively carve off 1, 2, and 3 steps,
/// with a memoizing hashmap and helper-function.
pub fn triple_step(steps: isize) -> isize {
    let mut map = HashMap::new();
    helper(steps, &mut map)
}

/// Recursive helper, including the caching hash-map
fn helper(steps: isize, cache: &mut HashMap<isize, isize>) -> isize {
    if steps < 0 {
        return 0;
    } else if steps == 0 {
        return 1;
    } else if cache.contains_key(&steps) {
        return cache.get(&steps).unwrap().clone();
    }
    // No base-cases checked out. Recurse and do some real work.
    let rv = helper(steps - 1, cache) + helper(steps - 2, cache) + helper(steps - 3, cache);
    // Store the result in our cache, and return it.
    cache.insert(steps, rv);
    rv
}

#[test]
fn test_triple_step() {
    let test_cases = [(-1, 0), (0, 1), (1, 1), (2, 2), (3, 4)];
    for case in test_cases {
        assert_eq!(triple_step(case.0), case.1);
    }
}
