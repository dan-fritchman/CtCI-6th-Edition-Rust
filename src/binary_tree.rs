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
impl<T: Clone> BinaryTree<T> {
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
