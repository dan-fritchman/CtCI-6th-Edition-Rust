//!
//! # Binary Tree Module
//!
//! Connections use the crate-level [Ptr] pointer-type, a wrapper over `Rc<RefCell>`.
//!

// Local Imports
use crate::ptr::Ptr;

// Alias for a [Ptr] to a [Node]
pub type NodePtr<T> = Ptr<Node<T>>;

/// Binary Tree Node
#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub left: Option<NodePtr<T>>,
    pub right: Option<NodePtr<T>>,
}
impl<T> Node<T> {
    pub fn new(data: T, left: Option<NodePtr<T>>, right: Option<NodePtr<T>>) -> Self {
        Self { data, left, right }
    }
}
/// Binary Tree
#[derive(Debug)]
pub struct BinaryTree<T> {
    pub head: Option<NodePtr<T>>,
}
impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self { head: None }
    }
}
impl<T: Clone> BinaryTree<T> {
    /// Insert a new element with value `data`.
    /// At the end of `insert`, the new element is always childless.
    /// It is appended as a child of `parent` if a slot is available.
    /// If a `parent` is provided, but has neither child available, returns an `Err`.
    /// If no `parent` is provided, and the tree has an existing `head`, also returns an `Err`.
    pub fn insert(&mut self, data: T, parent: Option<NodePtr<T>>) -> Result<NodePtr<T>, Error> {
        let ptr = NodePtr::new(Node::new(data, None, None));
        match parent {
            Some(ref p) => {
                let mut pptr = p.borrow_mut();
                if pptr.left.is_none() {
                    pptr.left = Some(ptr.clone());
                } else if pptr.right.is_none() {
                    pptr.right = Some(ptr.clone());
                } else {
                    return Err(Error); // Parent with no child-slots available. Fail.
                }
            }
            None => {
                match self.head {
                    Some(_) => return Err(Error), // No parent, and `head` taken. Fail.
                    None => self.head = Some(ptr.clone()),
                }
            }
        }
        Ok(ptr)
    }
    /// In-order traversal, creating and returning a vector of data-elements
    pub fn inorder(&self) -> Vec<T> {
        let mut v = Vec::new();
        self._inorder(&self.head, &mut v);
        v
    }
    /// Recursive helper for `inorder`
    fn _inorder(&self, opt: &Option<NodePtr<T>>, v: &mut Vec<T>) {
        match opt {
            None => (),
            Some(ptr) => {
                let node = &ptr.borrow();
                self._inorder(&node.left, v);
                v.push(node.data.clone());
                self._inorder(&node.right, v);
            }
        }
    }
}

/// Local Error Type
#[derive(Debug, PartialEq, Eq)]
pub struct Error;
