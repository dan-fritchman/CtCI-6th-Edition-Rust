//!
//! # Binary (Min) Heap
//!

/// # Binary (Min) Heap
///
/// Built on Rust's standard-library [Vec].
///
#[derive(Debug)]
pub struct Heap<T> {
    data: Vec<T>,
}
impl<T> Default for Heap<T> {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}
impl<T: Ord> Heap<T> {
    /// Push an element onto the heap
    pub fn push(&mut self, t: T) {
        // Push to the end of `data`
        self.data.push(t);
        // And re-establish the heap property
        self.siftup(self.data.len() - 1)
    }
    /// Pop the min element from the heap. Returns `None` if empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        // Swap the last element to our head
        let last = self.data.len() - 1;
        self.data.swap(0, last);

        // Pop the return/ head-element off the end
        let rv = self.data.pop().unwrap();

        if !self.data.is_empty() {
            self.siftdown(0); // Get to reorganizing
        }

        Some(rv)
    }
    /// Sift downward from index `idx`. Re-arranges children during insertion.
    fn siftdown(&mut self, idx: usize) {
        let val = &self.data[idx];
        let left = self.data.get(2 * idx + 1);
        let right = self.data.get(2 * idx + 2);
        match (left, right) {
            (None, None) => return, // No children, nothing to swap
            (Some(lf), None) => {
                // Left but no right child. Swap if out of order.
                if lf < &self.data[idx] {
                    self.data.swap(2 * idx + 1, idx);
                }
            }
            (Some(lf), Some(rt)) => {
                // Two children, potentially recursively update
                let min = lf.min(rt).min(val);
                if min == val {
                    return; // Min on top, already done
                } else if min == lf {
                    // Left-child is the min. Swap with it, and recursively sift down again.
                    self.data.swap(2 * idx + 1, idx);
                    return self.siftdown(2 * idx + 1);
                } else {
                    // Right-child is the min. Swap with it, and recursively sift down again.
                    self.data.swap(2 * idx + 2, idx);
                    return self.siftdown(2 * idx + 2);
                }
            }
            _ => unreachable!("Internal Error"), // The fourth case - right child but no left - shouldn't be possible
        };
    }
    /// Sift upwards from `idx`. Re-arranges parents during removal.
    fn siftup(&mut self, idx: usize) {
        if idx == 0 {
            return; // At root/ head node, no parents to swap with
        }
        let val = &self.data[idx];
        let parent = &self.data[(idx - 1) / 2];
        if val < parent {
            self.data.swap(idx, (idx - 1) / 2);
            return self.siftup((idx - 1) / 2);
        }
    }
}

#[test]
fn test_heap1() {
    let mut heap = Heap::default();

    heap.push(1);
    heap.push(2);
    heap.push(3);

    assert_eq!(heap.data.len(), 3);
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.data.len(), 2);
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.data.len(), 1);
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.data.len(), 0);
    assert_eq!(heap.pop(), None);
}

#[test]
fn test_heap2() {
    let mut heap = Heap::default();

    heap.push(3);
    heap.push(2);
    heap.push(1);

    assert_eq!(heap.data.len(), 3);
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.data.len(), 2);
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.data.len(), 1);
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.data.len(), 0);
    assert_eq!(heap.pop(), None);
}

#[test]
fn test_heap3() {
    let mut heap = Heap::default();

    heap.push(2);
    heap.push(3);
    heap.push(1);

    assert_eq!(heap.data.len(), 3);
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.data.len(), 2);
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.data.len(), 1);
    assert_eq!(heap.pop(), Some(3));
    assert_eq!(heap.data.len(), 0);
    assert_eq!(heap.pop(), None);
}

#[test]
fn test_heap4() {
    let mut heap = Heap::default();

    for k in (0..1_000).rev() {
        heap.push(k);
    }

    assert_eq!(heap.data.len(), 1_000);
    let mut min = 0;
    while let Some(k) = heap.pop() {
        assert_eq!(k, min);
        min += 1;
    }
    assert_eq!(heap.pop(), None);
}
