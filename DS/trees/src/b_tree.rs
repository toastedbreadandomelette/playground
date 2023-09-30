use crate::common::TreeInsOrder;
use core::{
    cell::RefCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
};
extern crate alloc;
use alloc::rc::Rc;

type Link<T> = Rc<RefCell<BNode<T>>>;
type MaybeLink<T> = Option<Link<T>>;

/// Node struct of generalized type `T`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BNode<T> {
    /// Actual data
    pub data: Vec<T>,
    /// Length
    pub len: usize,
    /// Left Node: can be either set to `None` or Shared `Node<T>`
    pub child_links: Vec<MaybeLink<T>>,
}

impl<T> BNode<T> {
    /// Create a new node
    pub fn new(elem: T, len: usize) -> Self {
        let mut list = Vec::with_capacity(len);
        list.push(elem);
        Self {
            data: list,
            len,
            child_links: Vec::with_capacity(len + 1),
        }
    }

    /// Returns `true` when node is a leaf
    ///
    /// Leaf node implies `left` and `right` branch is [`None`]
    #[inline(always)]
    pub fn is_leaf(&self) -> bool {
        self.child_links.iter().all(|c| c.is_none())
    }

    /// Returns whether function has child nodes
    ///
    /// Leaf node implies `left` and `right` branch is [`Some(Link<T>)`]
    #[inline(always)]
    pub fn has_all_children(&self) -> bool {
        self.child_links.iter().all(|c| c.is_some())
    }

    /// Return sizeof current node in bytes, plus
    /// all size from sub-branches
    pub fn size_of(&self) -> usize {
        core::mem::size_of::<Self>()
            + self.child_links.iter().fold(0, |prev, child| {
                prev + match child {
                    Some(link) => link.borrow().size_of(),
                    None => 0,
                }
            })
    }
}

impl<T> Deref for BNode<T>
where
    T: Deref<Target = T>,
{
    type Target = Self;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        self
    }
}

impl<T> DerefMut for BNode<T>
where
    T: DerefMut<Target = T>,
{
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

/// Binary Search Tree implementation
/// Stores the value of generic type `T`.
///
/// ### Condition for type `T`
///
/// should Implement [`PartialEq`], [`PartialOrd`], [`Clone`], [`Debug`]
#[derive(Debug, Clone)]
pub struct BTree<T>
where
    T: Clone + PartialOrd + PartialEq + Debug,
{
    /// Root of the node as `MaybeLink<T>`, `None` implies there are no nodes
    root: MaybeLink<T>,
    /// Comparison function: This decides the traversal, insertion and
    /// find algorithm
    // cmp: fn(&T, &T) -> TreeInsOrder,
    /// Size of the tree
    cell_len: usize,
    ///
    len: usize,
}

impl<T> BTree<T>
where
    T: Clone + PartialEq + PartialOrd + Debug,
{
    pub fn new(cell_len: usize) -> Self {
        Self {
            root: None,
            cell_len,
            len: 0,
        }
    }

    /// Internal: Capsule data as [`Rc`] and [`RefCell`]
    /// for dynamic mutability
    #[inline(always)]
    fn capsule(data: T, cell_len: usize) -> MaybeLink<T> {
        Some(Rc::new(RefCell::new(BNode::new(data, cell_len))))
    }
}
