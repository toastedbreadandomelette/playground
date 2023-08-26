use crate::common::TreeInsOrder;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

/// Logical color
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    /// Color Red
    Red,
    /// Color Black
    Black,
}

type RBLink<T> = Rc<RefCell<RBNode<T>>>;
type MaybeRBLink<T> = Option<RBLink<T>>;

/// Node struct of generalized type `T`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RBNode<T> {
    /// Actual data
    pub data: Box<T>,
    /// Color code
    pub color: Color,
    /// Left Node: can be either set to `None` or Shared `Node<T>`
    pub left: MaybeRBLink<T>,
    /// Right Node: can be either set to `None` or Shared `Node<T>`
    pub right: MaybeRBLink<T>,
}

impl<T> RBNode<T> {
    /// Create a new node
    pub fn new(data: T) -> Self {
        Self {
            data: Box::new(data),
            color: Color::Black,
            left: None,
            right: None,
        }
    }

    /// Return sizeof current node in bytes, plus
    ///
    /// all size from sub-branches
    pub fn size_of(&self) -> usize {
        std::mem::size_of::<Self>()
            + match &self.left {
                Some(left) => left.borrow().size_of(),
                None => 0,
            }
            + match &self.right {
                Some(right) => right.borrow().size_of(),
                None => 0,
            }
    }
}

/// Binary Search Tree implementation
#[derive(Debug, Clone)]
pub struct RBTree<T>
where
    T: Clone + PartialOrd + PartialEq + Debug,
{
    root: Option<Rc<RefCell<RBNode<T>>>>,
    cmp: fn(&T, &T) -> TreeInsOrder,
    len: usize,
}

impl<T> RBTree<T>
where
    T: Clone + PartialOrd + PartialEq + Debug,
{
    pub fn new(fun: Option<fn(&T, &T) -> TreeInsOrder>) -> Self {
        let cmp = if let Some(function) = fun {
            function
        } else {
            |parent: &T, child: &T| {
                if parent > child {
                    TreeInsOrder::Left
                } else if parent < child {
                    TreeInsOrder::Right
                } else {
                    TreeInsOrder::Eq
                }
            }
        };
        Self {
            root: None,
            cmp,
            len: 0,
        }
    }
}