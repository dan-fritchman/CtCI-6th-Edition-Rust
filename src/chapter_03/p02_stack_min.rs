//!
//! # Stack Min:
//!
//! How would you design a stack which, in addition to push and pop, has a function min which returns the minimum element?
//! Push, pop and min should all operate in 0(1) time.
//!
//! Hints: #27, #59, #78
//!

/// "Stack" which internally keeps a separate min-values stack
#[derive(Default)]
pub struct StackWithMins<T> {
    vec: Vec<T>,
    mins: Vec<T>,
}
impl<T: Clone + PartialOrd> StackWithMins<T> {
    pub fn push(&mut self, t: T) {
        // If `t` is le our current min, add a copy to the min-stack
        match self.min() {
            Some(min) if t <= *min => self.mins.push(t.clone()),
            None => self.mins.push(t.clone()),
            _ => (),
        }
        // And push it onto the main stack
        self.vec.push(t)
    }
    pub fn pop(&mut self) -> Option<T> {
        // Pop the top element
        let rv = self.vec.pop()?;
        // And if it equals the min, pop the min-stack too.
        // Note in order to survive the `?` and be here, `min` must have an entry.
        if rv <= *self.min().unwrap() {
            self.mins.pop();
        }
        Some(rv)
    }
    pub fn peek(&self) -> Option<&T> {
        self.vec.last()
    }
    pub fn min(&mut self) -> Option<&T> {
        self.mins.last()
    }
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}

#[test]
fn test_stack_with_mins() {
    let mut s = StackWithMins::default();
    assert_eq!(s.min(), None);
    s.push(1);
    s.push(2);
    s.push(3);
    assert_eq!(s.min().cloned(), Some(1));
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.min().cloned(), Some(1));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.min().cloned(), Some(1));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.min(), None);

    let mut s = StackWithMins::default();
    assert_eq!(s.min(), None);
    s.push(3);
    s.push(2);
    s.push(1);
    assert_eq!(s.min().cloned(), Some(1));
    assert_eq!(s.pop(), Some(1));
    assert_eq!(s.min().cloned(), Some(2));
    assert_eq!(s.pop(), Some(2));
    assert_eq!(s.min().cloned(), Some(3));
    assert_eq!(s.pop(), Some(3));
    assert_eq!(s.min(), None);
}
