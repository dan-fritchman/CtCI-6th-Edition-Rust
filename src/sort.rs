//!
//! # Sorting Module
//!
//! Survey of popular methods, heavily inspired by @jonhoo's "Crust of Rust" treatment.
//!

/// Sample using the standard library's [slice::sort]
pub struct StdLibSort;
impl StdLibSort {
    pub fn sort<T: Ord>(items: &mut [T]) {
        items.sort()
    }
}

#[test]
fn test_std() {
    let mut items = vec![10, 0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
    StdLibSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

///
/// # Bubble Sort
///
pub struct BubbleSort;
impl BubbleSort {
    pub fn sort<T: Ord>(items: &mut [T]) {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for idx in 1..items.len() {
                if items[idx - 1] > items[idx] {
                    items.swap(idx - 1, idx);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn test_bubble() {
    let mut items = vec![10, 0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
    BubbleSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

///
/// # Selection Sort
///
pub struct SelectionSort;
impl SelectionSort {
    pub fn sort<T: Ord>(items: &mut [T]) {
        // For each index, swap it with the minimum
        for idx in 0..(items.len() - 1) {
            // Get the min over the remaining elements
            let (m, _) = items[idx..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v)
                .unwrap();
            // Now, that index is into `items[idx..]`, which starts at `idx`
            let m = m + idx;
            // And swap the element at `idx` with the min
            items.swap(idx, m);
        }
    }
}

#[test]
fn test_selection() {
    let mut items = vec![10, 0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
    SelectionSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

///
/// # Insertion Sort
///
pub struct InsertionSort;
impl InsertionSort {
    pub fn sort<T: Ord>(items: &mut [T]) {
        for partition in 1..items.len() {
            let mut i = partition;
            while i > 0 && items[i - 1] > items[i] {
                items.swap(i - 1, i);
                i -= 1;
            }
        }
    }
}

#[test]
fn test_insertion() {
    let mut items = vec![10, 0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
    InsertionSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

///
/// # Insertion Sort, with Binary Search
///
pub struct InsertionSortBinary;
impl InsertionSortBinary {
    pub fn sort<T: Ord>(items: &mut [T]) {
        for partition in 1..items.len() {
            // Find the index in the sorted-half where the partition-element belongs
            let idx = match &items[..partition].binary_search(&items[partition]) {
                // `binary_search` returns `Ok` on an exact hit, `Err` if not, but the correct index either way.
                Ok(idx) | Err(idx) => *idx,
            };
            // And rotate it into place
            items[idx..partition + 1].rotate_right(idx);
        }
    }
}

#[test]
fn test_insertion_binary() {
    let mut items = vec![10, 0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
    InsertionSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

///
/// # Quick Sort
///
pub struct QuickSort;
impl QuickSort {
    pub fn sort<T: Ord>(items: &mut [T]) {
        // Call the recursive helper
        QuickSort::helper(items)
    }
}
impl QuickSort {
    /// Recursive quicksort implementation
    fn helper<T: Ord>(items: &mut [T]) {
        match items.len() {
            0 | 1 => return,
            2 => {
                if items[0] > items[1] {
                    items.swap(0, 1);
                }
                return;
            }
            _ => (),
        };
        // Split out the first element as our pivot
        let (pivot, rest) = items.split_first_mut().unwrap();

        // Create the left & right index-trackers
        let mut left = 0;
        let mut right = rest.len() - 1;

        // Make all of our pivot-swaps
        while left <= right {
            if rest[left] <= *pivot {
                left += 1;
            } else if rest[right] > *pivot {
                if right == 0 {
                    break; // Done, all went to the right
                }
                right -= 1;
            } else {
                rest.swap(left, right);
                left += 1;
                if right == 0 {
                    break; // Done, all went to the right
                }
                right -= 1;
            }
        }
        // Swap the pivot to its correct location
        items.swap(0, left);

        // Split into two sub-slices, and recursively quicksort them
        let (left, right) = items.split_at_mut(left);
        Self::helper(left);
        Self::helper(&mut right[1..]);
    }
}

#[test]
fn test_quick() {
    let mut items = vec![10, 0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
    QuickSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}

///
/// # Merge Sort
///
pub struct MergeSort;
impl MergeSort {
    /// Primary implementation.
    /// Creates a copy of `items` and begins recursive calls.
    pub fn sort<T: Ord + Clone + Default>(items: &mut [T]) {
        let mut cp = Self::copy(items);
        Self::mergesort(items, &mut cp)
    }
    /// Recursive merge-sort helper routine
    fn mergesort<T: Ord + Clone + Default>(a: &mut [T], b: &mut [T]) {
        match a.len() {
            0 | 1 => return,
            2 => {
                if a[0] > a[1] {
                    a.swap(0, 1);
                }
                return;
            }
            _ => (),
        };

        // Split each of `a` and `b` in halves
        let a_halves = a.split_at_mut(a.len() / 2);
        let b_halves = b.split_at_mut(b.len() / 2);

        // Merge-sort each half, swapping roles on each recursive call
        Self::mergesort(b_halves.0, a_halves.0); // Left
        Self::mergesort(b_halves.1, a_halves.1); // Right
        drop(a_halves);

        // Merge the two halves of `a` into `b`
        Self::merge(b_halves.0, b_halves.1, a);
    }
    /// Merge `left` and `right` into `dest`.
    /// Requires that len(left) + len(right) == len(dest), or panics.
    fn merge<T: Ord + Clone + Default>(left: &[T], right: &[T], dest: &mut [T]) {
        let mut lp = 0;
        let mut rp = 0;
        for dp in 0..dest.len() {
            if lp < left.len() && (rp >= right.len() || left[lp] < right[rp]) {
                dest[dp] = left[lp].clone();
                lp += 1;
            } else {
                dest[dp] = right[rp].clone();
                rp += 1;
            }
        }
    }
    /// Copy `items` to a new [Vec]
    fn copy<T: Ord + Clone + Default>(items: &mut [T]) -> Vec<T> {
        items.iter().map(|i| i.clone()).collect::<Vec<T>>()
    }
}

#[test]
fn test_merge() {
    let mut items = vec![10, 0, 9, 1, 8, 2, 7, 3, 6, 4, 5];
    MergeSort::sort(&mut items);
    assert_eq!(items, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
}
