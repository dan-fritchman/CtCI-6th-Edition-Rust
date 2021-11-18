//!
//! # Intersection:
//!
//! Given two (singly) linked lists, determine if the two lists intersect.
//! Return the intersecting node. Note that the intersection is defined based on reference, not value.
//! That is, if the kth node of the first linked list is the exact same node (by reference)
//! as the jth node of the second linked list, then they are intersecting.
//!
//! Hints: #20, #45, #55, #65, #76, #93, #111, #120, #129
//!

// We can't really do this one with our "slot-map-style" Rustic implementation.
// So, pull in the shared-pointer [Ptr] and make a new [List] instead.

use std::{collections::HashSet, marker::PhantomData};

// Local Imports
use crate::ptr::Ptr;

#[derive(Debug)]
pub struct Node<Data = isize> {
    data: Data,
    next: Option<Ptr<Node>>,
}
#[derive(Debug)]
pub struct List {
    head: Option<Ptr<Node>>,
    tail: Option<Ptr<Node>>,
}
impl Default for List {
    fn default() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }
}
impl List {
    /// Add a node with value `data` to the end of the list.
    pub fn add(&mut self, data: isize) -> Ptr<Node> {
        let node = Node { data, next: None };
        let ptr = Ptr::new(node);
        match self.head {
            Some(_) => (),
            None => self.head = Some(ptr.clone()),
        };
        match self.tail {
            Some(ref mut t) => t.borrow_mut().next = Some(ptr.clone()),
            None => (),
        };
        self.tail = Some(ptr.clone());
        ptr
    }
    /// Add each element of a slice
    pub fn add_slice(&mut self, sl: &[isize]) {
        for item in sl {
            self.add(*item);
        }
    }
    /// Create from a slice or vector of [isize]
    pub fn from_slice(sl: &[isize]) -> Self {
        let mut this = Self::default();
        this.add_slice(sl);
        this
    }
    pub fn iter(&self) -> ListIter {
        ListIter::new(self)
    }
}

/// # List Iterator
/// For our ergonomic niceties, we might as well make these iterators! (See below.)
pub struct ListIter<'ls> {
    /// Current node (pointer)
    current: Option<Ptr<Node>>,
    /// Hold a (phantom) reference to the list,
    /// so that it may not be modified during our lifetime.
    list: PhantomData<&'ls List>,
}
impl<'ls> ListIter<'ls> {
    /// Create a [ListIter] from a [List]
    pub fn new(list: &'ls List) -> Self {
        Self {
            list: PhantomData,
            current: list.head.clone(),
        }
    }
}
impl<'ls> Iterator for ListIter<'ls> {
    type Item = Ptr<Node>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut next = match self.current {
            Some(ref p) => p.borrow().next.clone(),
            None => return None,
        };
        std::mem::swap(&mut self.current, &mut next);
        next
    }
}

/// Primary Implementation
///
/// Insert each element of `a` into a [HashSet],
/// then check whether it contains any elements of `b`.
///
pub fn intersection(a: &List, b: &List) -> Option<Ptr<Node>> {
    // Insert each node in `a` into a hash-set
    let mut aset = HashSet::new();

    // (How cool is that `impl Iterator` now.)
    for ptr in a.iter() {
        aset.insert(ptr.clone());
    }

    // And check whether it contains each element in `b`
    for ptr in b.iter() {
        if aset.contains(&ptr) {
            return Some(ptr.clone());
        }
    }
    None // No intersection found
}

#[test]
fn test_intersecting() {
    // Create two (initially non-intersecting) lists
    let mut a = List::from_slice(&[10, 11, 12, 13, 14, 15]);
    let mut b = List::from_slice(&[20, 21, 22]);

    // Check that they do not intersect
    assert_eq!(intersection(&a, &b), None);

    // Now create a shared sub-list
    let mut shared = List::default();
    let shared_head = shared.add(1);
    shared.add_slice(&[2, 3, 4]);

    // (Illegally) link `shared` to the end of both `a` and `b`
    a.tail.as_mut().unwrap().borrow_mut().next = shared.head.clone();
    a.tail = shared.tail.clone();
    b.tail.as_mut().unwrap().borrow_mut().next = shared.head.clone();
    b.tail = shared.tail.clone();

    // Check the head of `shared` comes back as the result
    assert_eq!(intersection(&a, &b), Some(shared_head));
}
