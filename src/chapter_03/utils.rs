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
impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { vec: Vec::new() }
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
