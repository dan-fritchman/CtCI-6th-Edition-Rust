//!
//! # Shortest Supersequence:
//!
//! You are given two arrays, one shorter (with all distinct elements) and one longer.
//! Find the shortest subarray in the longer array that contains all the elements in the shorter array.
//! The items can appear in any order.
//!
//! EXAMPLE
//! Input: {1, 5, 9}, {7, 5, 9, 13, 2, 1, 3, (5, 7, 9,), 1, 1, 5, 8, 8, 9, 7}
//! Output: [7, 10] (the underlined portion above)
//!
//! Hints: #645, #652, #669, #681, #691, #725, #731, #741
//!

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

/// Primary Implementation
///
/// Create a list of occurrences for each entry in `short`,
/// then priority-walk over them, tracing the smallest (max-min) [Range].
///
pub fn shortest_supersequence(long: &[isize], short: &[isize]) -> Option<Range> {
    // Walk through `long` for each element of `short`, creating a list of its indices
    let mut locs: HashMap<isize, Vec<usize>> = HashMap::new();
    for se in short.iter() {
        let mut se_locs = Vec::new();
        for (lidx, le) in long.iter().enumerate() {
            if se == le {
                se_locs.push(lidx);
            }
        }
        if se_locs.is_empty() {
            return None; // If the element didn't show up, there's no set, we're done.
        }
        // Add its locations to the map
        locs.insert(*se, se_locs);
    }

    // Create a min-heap, and insert the first elements from each list.
    let mut heap: BinaryHeap<HeapEntry> = BinaryHeap::new();
    let mut range = Range {
        min: usize::MAX,
        max: usize::MIN,
    };
    for (value, list) in locs.iter_mut() {
        let index = list.remove(0);
        range.max = range.max.max(index);
        range.min = range.min.min(index);
        heap.push(HeapEntry::new(*value, index));
    }

    // Initialize the minimum [Range]
    let mut min_range = range.clone();

    while let Some(min_entry) = heap.pop() {
        range.min = min_entry.index;

        // If this shrinks our existing range, replace the running minimum
        if range.max - range.min < min_range.max - min_range.min {
            min_range = range.clone();
        }
        // If there are more entries remaining for this value, remove one from the pending `locs` and push it to the queue,
        // potentially update the range's max value.
        let list = locs.get_mut(&min_entry.value).unwrap();
        if list.is_empty() {
            break; // No more sub-sequences, we done.
        }
        let pop = list.remove(0);
        range.max = range.max.max(pop);
        heap.push(HeapEntry::new(min_entry.value, pop));
    }

    // Queue empty - this is as good as it gets!
    Some(min_range)
}

/// # Range
/// Min and max indices-pair
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Range {
    min: usize,
    max: usize,
}

/// Entry in the min-heap.
/// Compares based on `index` field.
/// Inverts comparison order to make the std-lib max-heap instead produce the min.
#[derive(Debug, Default)]
pub struct HeapEntry {
    value: isize,
    index: usize,
}
impl HeapEntry {
    fn new(value: isize, index: usize) -> Self {
        Self { value, index }
    }
}
impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        other.index.eq(&self.index)
    }
}
impl Eq for HeapEntry {}
impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.index.partial_cmp(&self.index)
    }
}
impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        other.index.cmp(&self.index)
    }
}
#[test]
fn test_shortest_supersequence() {
    // Closure to convert a numeric string to a vector of numbers
    let ize = |s: &str| {
        s.chars()
            .map(|c| c.to_digit(10).unwrap() as isize)
            .collect::<Vec<isize>>()
    };
    let s = ize(&"75902135791158897");
    let t = ize(&"159");
    assert_eq!(
        shortest_supersequence(&s, &t),
        Some(Range { min: 7, max: 10 })
    );
}
