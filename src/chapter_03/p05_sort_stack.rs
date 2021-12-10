//!
//! # Sort Stack:
//!
//! Write a program to sort a stack such that the smallest items are on the top.
//! You can use an additional temporary stack, but you may not copy the elements
//! into any other data structure (such as an array).
//! The stack supports the following operations: push, pop, peek, and is_empty.
//!
//! Hints: # 75, #32, #43
//!

// Local Imports
use super::utils::Stack;

/// Primary Implementation
pub fn sort_stack<T: PartialOrd>(stack: &mut Stack<T>) {
    // Create the aux-stack
    let mut other = Stack::new();

    // For each element in `stack`, move it to `other`, at the current "correct" place in order.
    // All elements coming after the element are temporarily pushed back to `stack`.
    while !stack.is_empty() {
        let val = stack.pop().unwrap();
        while !other.is_empty() && *other.peek().unwrap() > val {
            stack.push(other.pop().unwrap());
        }
        other.push(val);
    }
    // At this point `stack` is empty, and `other` is in descending order.
    // Reverse the order of elements back into `stack`.
    while !other.is_empty() {
        stack.push(other.pop().unwrap());
    }
}

#[test]
fn test_sort_stack() {
    let mut stack = Stack::new();
    stack.push(1);
    sort_stack(&mut stack);
    assert_eq!(stack.len(), 1);

    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    sort_stack(&mut stack);
    assert_eq!(stack.len(), 2);

    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    sort_stack(&mut stack);
    assert_eq!(stack.len(), 3);

    let mut stack = Stack::new();
    stack.push(1);
    sort_stack(&mut stack);
    assert_eq!(stack.pop(), Some(1));

    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    sort_stack(&mut stack);
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(2));

    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    sort_stack(&mut stack);
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(3));

    let mut stack = Stack::new();
    stack.push(3);
    stack.push(2);
    stack.push(1);
    stack.push(4);
    sort_stack(&mut stack);
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(4));
}
