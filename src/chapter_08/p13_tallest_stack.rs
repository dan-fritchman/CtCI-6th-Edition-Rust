//!
//! # Stack of Boxes:
//!
//! You have a stack of n boxes, with widths `wi` heights `hi` and depths `di`.
//! The boxes cannot be rotated and can only be stacked on top of one another
//! if each box in the stack is strictly larger than the box above it in width, height, and depth.
//! Implement a method to compute the height of the tallest possible stack.
//! The height of a stack is the sum of the heights of each box.
//!
//! Hints: #155, #194, #214, #260, #322, #368, #378
//!

use std::collections::HashMap;

/// # Box
/// Like, a cardboard box, not Rust's standard-library heap-pointer type.
#[derive(Debug, Clone)]
pub struct Box {
    height: usize,
    width: usize,
    depth: usize,
}
impl Box {
    pub fn new(height: usize, width: usize, depth: usize) -> Self {
        Self {
            height,
            width,
            depth,
        }
    }
}

/// Primary Implementation
///
/// Sort the boxes by one dimension, then walk up that sorted list,
/// recursively finding the tallest stack starting with each box.
///
pub fn tallest_stack(boxes: &mut [Box]) -> usize {
    // First sort the boxes by height, as a shortcut for which we have to traverse
    boxes.sort_unstable_by(|a, b| a.height.cmp(&b.height));

    // Create our intermediate results-cache
    let mut cache = HashMap::new();

    // And check starting from each box
    let mut best = 0;
    for idx in 0..boxes.len() {
        let its = helper(&boxes, idx, &mut cache);
        best = best.max(its);
    }
    best
}

/// Recursive helper.
/// Returns the max-height starting from box-index `idx`.
fn helper(boxes: &[Box], idx: usize, cache: &mut HashMap<usize, usize>) -> usize {
    if cache.contains_key(&idx) {
        return *cache.get(&idx).unwrap(); // Already done
    }
    // Do some real work. Try each remaining box, checking its max stack-height.
    let cmp = &boxes[idx];
    let mut best = 0;
    for nextidx in idx..boxes.len() {
        let rem = &boxes[nextidx];
        if rem.width > cmp.width && rem.depth > cmp.depth && rem.height > cmp.height {
            // Recursively get the best height of the remaining boxes
            let its = helper(boxes, nextidx, cache);
            // And merge it into our best value
            best = best.max(its);
        }
    }
    cache.insert(idx, best + cmp.height);
    best + cmp.height
}

#[test]
fn test_tallest_stack() {
    assert_eq!(tallest_stack(&mut []), 0);
    assert_eq!(tallest_stack(&mut [Box::new(3, 2, 1)]), 3);
    assert_eq!(
        tallest_stack(&mut [Box::new(3, 2, 1), Box::new(5, 4, 1)]),
        5
    );
    assert_eq!(
        tallest_stack(&mut [Box::new(3, 2, 1), Box::new(6, 5, 4)]),
        9
    );
}
