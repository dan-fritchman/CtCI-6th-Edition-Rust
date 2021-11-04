//!
//! # Queue via Stacks:
//!
//! Implement a MyQueue class which implements a queue using two stacks.
//!
//! Hints: #98, #7 74
//!

// Local Imports
use super::utils::Stack;

/// Queue comprised of two stacks
///
/// Calls to `push` are stacked onto `new`, which is in stack-order (LIFO).
/// Calls to `pop` are pulled from `old`, which is in queue-order (FIFO).
/// If `old` is empty, all elements of `new` are popped and pushed into `old`, reversing their order.
///
pub struct MyQueue<T> {
    new: Stack<T>,
    old: Stack<T>,
}
impl<T> MyQueue<T> {
    pub fn new() -> Self {
        Self {
            new: Stack::new(),
            old: Stack::new(),
        }
    }
    pub fn push(&mut self, t: T) {
        self.new.push(t)
    }
    pub fn pop(&mut self) -> Option<T> {
        self.maybe_transfer();
        self.old.pop()
    }
    /// Peek at the "head" element
    /// Note peeking may require re-shuffling internal data,
    /// and therefore requires `&mut self`.
    pub fn peek(&mut self) -> Option<&T> {
        self.maybe_transfer();
        self.old.peek()
    }
    /// Transfer everything from `new` to `old`, in reverse order, if `old` is empty.
    fn maybe_transfer(&mut self) {
        if self.old.is_empty() {
            while let Some(t) = self.new.pop() {
                self.old.push(t);
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.new.is_empty() && self.old.is_empty()
    }
    pub fn len(&self) -> usize {
        self.new.len() + self.old.len()
    }
}

#[test]
fn test_myqueue() {
    let test_cases = [
        vec![1, 2, 3],
        vec![-1, 0, 1],
        vec![5, 7, 11, 13, 17, 19, 23],
    ];

    for case in test_cases.iter() {
        let mut q = MyQueue::new();
        for (index, val) in case.iter().enumerate() {
            q.push(*val);
            assert_eq!(q.len(), index + 1);
        }
        for (index, _val) in case.iter().enumerate() {
            q.pop();
            assert_eq!(q.len(), case.len() - index - 1);
        }
    }

    for case in test_cases.iter() {
        let mut q = MyQueue::new();
        for val in case.iter() {
            q.push(*val);
            assert_eq!(q.peek().unwrap().clone(), case[0]);
        }
        assert_eq!(q.len(), case.len())
    }

    for case in test_cases.iter() {
        let mut q = MyQueue::new();
        for val in case.iter() {
            q.push(*val)
        }
        assert_eq!(q.old.len(), 0);
        assert_eq!(q.new.len(), case.len());
        assert_eq!(q.new.peek().unwrap().clone(), case[case.len() - 1]);

        // Invoke the (private) transfer-method
        q.maybe_transfer();

        // And check some (private) internal state
        assert_eq!(q.old.len(), case.len());
        assert_eq!(q.new.len(), 0);
        assert_eq!(q.old.peek().unwrap().clone(), case[0]);
    }

    for case in test_cases.iter() {
        let mut q = MyQueue::new();
        for val in case.iter() {
            q.push(*val);
            assert_eq!(q.peek().unwrap().clone(), case[0]);
        }
        q.pop();
        assert_eq!(q.peek().unwrap().clone(), case[1]);
    }

    for case in test_cases.iter() {
        let mut q = MyQueue::new();
        for val in case.iter() {
            q.push(*val);
        }
        for i in 0..case.len() {
            assert_eq!(q.pop(), Some(case[i]))
        }
    }

    let mut q = MyQueue::new();
    q.push(4);
    q.push(6);
    assert_eq!(q.peek().cloned(), Some(4));

    let mut q = MyQueue::new();
    q.push(4);
    q.push(6);
    assert_eq!(q.peek().cloned(), Some(4));
    q.push(101);
    assert_eq!(q.peek().cloned(), Some(4));

    let mut q = MyQueue::new();
    q.push(4);
    q.push(6);
    assert_eq!(q.len(), 2);
    assert_eq!(q.pop(), Some(4));
    assert_eq!(q.pop(), Some(6));
    assert_eq!(q.len(), 0);
    assert_eq!(q.pop(), None);
}
