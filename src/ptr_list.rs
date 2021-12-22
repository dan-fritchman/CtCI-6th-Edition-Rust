//!
//! # [Ptr]-Based Linked List
//!

use std::marker::PhantomData;

// Local Imports
use crate::ptr::Ptr;

#[derive(Debug)]
pub struct Node<Data = isize> {
    pub data: Data,
    pub next: Option<Ptr<Node>>,
}
#[derive(Debug, Default)]
pub struct List {
    pub head: Option<Ptr<Node>>,
    pub tail: Option<Ptr<Node>>,
}
impl List {
    /// Add a node with value `data` to the end of the list.
    pub fn add(&mut self, data: isize) -> Ptr<Node> {
        let node = Node { data, next: None };
        let ptr = Ptr::new(node);
        if self.head.is_none() {
            self.head = Some(ptr.clone());
        }
        if let Some(ref mut t) = self.tail {
            t.borrow_mut().next = Some(ptr.clone());
        }
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
    /// Create a [ListIter] iterator
    pub fn iter(&self) -> ListIter {
        ListIter::new(self)
    }
    /// Convert to a vector of data-elements
    pub fn to_vec(&self) -> Vec<isize> {
        let mut rv = Vec::new();
        for e in ListIter::new(self) {
            rv.push(e.borrow().data);
        }
        rv
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
