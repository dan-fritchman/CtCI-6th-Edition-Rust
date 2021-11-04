//!
//! # Stack of Plates:
//!
//! Imagine a (literal) stack of plates. If the stack gets too high, it might topple.
//! Therefore, in real life, we would likely start a new stack when the previous stack exceeds some threshold.
//! Implement a data structure SetOfStacks that mimics this. SetOfStacks should be composed of several stacks and should create a new stack once the previous one exceeds capacity.
//! SetOfStacks. push() and SetOfStacks.pop() should behave identically to a single stack
//! (that is, pop() should return the same values as it would if there were just a single stack).
//!
//! FOLLOW UP
//! Implement a function `popAt(int index)`
//! which performs a pop operation on a specific sub-stack.
//!
//! Hints: #64, #87
//!

use super::utils::Stack;

#[derive(Debug)]
pub struct PlateStack<T> {
    threshold: usize,
    stacks: Vec<Stack<T>>,
}
impl<T> PlateStack<T> {
    pub fn new(threshold: usize) -> Self {
        Self {
            stacks: vec![Stack::new()],
            threshold,
        }
    }
    pub fn push(&mut self, t: T) {
        // If the current stack is full, add a new one
        if self.last().len() == self.threshold {
            self.stacks.push(Stack::new());
        }
        // And push the element
        self.last_mut().push(t)
    }
    pub fn pop(&mut self) -> Option<T> {
        // Pop from the current stack
        let mut rv = self.last_mut().pop();
        // And if it was empty, *and* we have more than one stack, pop it
        if rv.is_none() && self.stacks.len() > 1 {
            self.stacks.pop();
            rv = self.last_mut().pop();
        }
        rv
    }
    pub fn peek(&self) -> Option<&T> {
        self.last().peek()
    }
    pub fn is_empty(&self) -> bool {
        self.stacks.len() == 1 && self.last().is_empty()
    }
    pub fn len(&self) -> usize {
        let mut rv = self.last().len();
        if self.stacks.len() > 1 {
            rv += self.threshold * (self.stacks.len() - 1)
        }
        rv
    }
    /// Accessors for our last [Stack]
    /// [PlateStack] builds-in the invariant that there is *always* at least one element in `self.stacks`.
    /// So values returned from `self.stacks.last()` can always be safely `unwrap()`ed.
    fn last(&self) -> &Stack<T> {
        self.stacks.last().unwrap()
    }
    fn last_mut(&mut self) -> &mut Stack<T> {
        self.stacks.last_mut().unwrap()
    }
}

#[test]
fn test_plate_stack() {
    let mut stack = PlateStack::new(3);
    for data in 0..11 {
        stack.push(data);
    }
    assert_eq!(stack.len(), 11);
    assert_eq!(stack.stacks.len(), 4);
    assert_eq!(stack.last().len(), 2);
    assert_eq!(stack.pop(), Some(10));
    assert_eq!(stack.pop(), Some(9));
    assert_eq!(stack.pop(), Some(8));
    assert_eq!(stack.pop(), Some(7));
    assert_eq!(stack.len(), 7);
    assert_eq!(stack.stacks.len(), 3);
    assert_eq!(stack.last().len(), 1);
}
