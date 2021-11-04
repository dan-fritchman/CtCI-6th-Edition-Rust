//!
//! # Chapter 3 Utilities and Definitions
//!
//! Defines the [Stack] and [Queue] structures used throughout.
//!
//! Rust's [std::collections] recommends:
//!
//! Use a [Vec] when:
//! * ...
//! * *You want a stack.*
//!
//! So, we take that advice.
//! Most [Stack] methods are just forwarded along to [Vec].
//!

#[derive(Debug)]
pub struct Stack<T> {
    vec: Vec<T>,
}
// Sadly this *is not* what [derive(Default)] generates,
// likely due to https://github.com/rust-lang/rust/issues/26925.
impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self { vec: Vec::new() }
    }
}
impl<T> Stack<T> {
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
