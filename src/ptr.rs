//!
//! # Shared Pointer [Ptr] Module
//!
//! A newtype-wrapper of `Rc<RefCell>` which also adds by-address implementations of [Eq] and [Hash].
//! The former aids in comparing nodes by identify rather than value,
//! and the latter aids in using hash-based structures to store, for example, sets of graph-nodes.
//!

use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    rc::Rc,
};

///
/// # Shared Pointer
///
/// Newtype wrapper over Rc<RefCell<T>>.
/// In addition to offering a shorter name, adds by-reference [Hash] and [Eq] implementations,
/// so that pointers can (a) be identity-compared, (b) be used in hash-sets (e.g. "previously seen" sets).
///
#[derive(Debug)]
pub struct Ptr<T>(Rc<RefCell<T>>);
impl<T> Ptr<T> {
    pub fn new(t: T) -> Self {
        Ptr(Rc::new(RefCell::new(t)))
    }
}

// Give these [Ptr]s address-based hashing and equality.
impl<T> Hash for Ptr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state)
    }
}
impl<T> PartialEq<Ptr<T>> for Ptr<T> {
    fn eq(&self, other: &Ptr<T>) -> bool {
        self.0.as_ptr() == other.0.as_ptr()
    }
}
impl<T> Eq for Ptr<T> {}

/// Deriving [Clone] and [Deref], for whatever reason, doesn't quite work
impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        Ptr(Rc::clone(&self.0))
    }
}
impl<T> Deref for Ptr<T> {
    type Target = Rc<RefCell<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for Ptr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
