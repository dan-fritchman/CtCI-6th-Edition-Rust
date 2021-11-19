//!
//! # Sort Stack:
//!
//! Write a program to sort a stack such that the smallest items are on the top.
//! You can use an additional temporary stack, but you may not copy the elements into any other data structure (such as an array).
//! The stack supports the following operations: push, pop, peek, and is_empty.
//!
//! Hints: # 75, #32, #43
//!

// Local Imports
use super::utils::Stack;

#[derive(Debug)]
pub struct SortedStack<T> {
    vec: Vec<T>,
}
// Sadly this *is not* what [derive(Default)] generates,
// likely due to https://github.com/rust-lang/rust/issues/26925.
impl<T> Default for SortedStack<T> {
    fn default() -> Self {
        Self { vec: Vec::new() }
    }
}
// FIXME! all this is just *regular* Stack. Implement it!
impl<T> SortedStack<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn push(&mut self, t: T) {
        self.vec.push(t)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
    pub fn peek(&self) -> Option<&T> {
        self.vec.last()
    }
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

pub fn sort_stack<T: PartialOrd>(_stack: &mut Stack<T>) {
    todo!()
}

#[ignore] // FIXME!
#[test]
fn test_sort_stack() {
    let mut queue = SortedStack::new();
    queue.push(1);
    assert_eq!(queue.len(), 1);

    let mut queue = SortedStack::new();
    queue.push(1);
    queue.push(2);
    assert_eq!(queue.len(), 2);

    let mut queue = SortedStack::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    assert_eq!(queue.len(), 3);

    let mut queue = SortedStack::new();
    queue.push(1);
    assert_eq!(queue.pop(), Some(1));

    let mut queue = SortedStack::new();
    queue.push(1);
    queue.push(2);
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));

    let mut queue = SortedStack::new();
    queue.push(1);
    queue.push(2);
    queue.push(3);
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));

    let mut queue = SortedStack::new();
    queue.push(3);
    queue.push(2);
    queue.push(1);
    queue.push(4);
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), Some(4));
}
