use crate::common::TreeInsOrder;
use std::{
    cell::RefCell,
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
};

type Link<T> = Rc<RefCell<Node<T>>>;
type MaybeLink<T> = Option<Link<T>>;

/// Node struct of generalized type `T`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node<T> {
    /// Actual data
    pub data: Box<T>,
    /// Left Node: can be either set to `None` or Shared `Node<T>`
    pub left: MaybeLink<T>,
    /// Right Node: can be either set to `None` or Shared `Node<T>`
    pub right: MaybeLink<T>,
}

impl<T> Node<T> {
    /// Create a new node
    pub fn new(data: T) -> Self {
        Self {
            data: Box::new(data),
            left: None,
            right: None,
        }
    }

    /// Returns `true` when node is a leaf
    ///
    /// Leaf node implies `left` and `right` branch is `None`
    #[inline(always)]
    pub fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }

    /// Returns whether function has child nodes
    ///
    /// Leaf node implies `left` and `right` branch is `Some(Link<T>)`
    #[inline(always)]
    pub fn has_both_children(&self) -> bool {
        self.left.is_some() && self.right.is_some()
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

impl<T> Deref for Node<T>
where
    T: Deref<Target = T>,
{
    type Target = Self;
    fn deref(&self) -> &Self::Target {
        self
    }
}

impl<T> DerefMut for Node<T>
where
    T: DerefMut<Target = T>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

/// Binary Search Tree implementation
/// Stores the value of generic type `T`.
///
/// ### Condition for type `T`
///
/// should Implement `PartialEq`, `PartialOrd`, `Clone`, `Debug`
#[derive(Debug, Clone)]
pub struct BSTree<T>
where
    T: Clone + PartialOrd + PartialEq + Debug,
{
    /// Root of the node as `MaybeLink<T>`, `None` implies there are no nodes
    root: MaybeLink<T>,
    /// Comparison function: This decides the traversal, insertion and 
    /// find algorithm
    cmp: fn(&T, &T) -> TreeInsOrder,
    /// Size of the tree
    len: usize,
}

impl<T> BSTree<T>
where
    T: Clone + PartialOrd + PartialEq + Debug,
{
    /// Create an empty Binary Search Tree
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

    /// Internal: Capsule data as `Rc` and `RefCell`
    /// for dynamic mutability
    #[inline(always)]
    fn capsule(data: T) -> MaybeLink<T> {
        Some(Rc::new(RefCell::new(Node::new(data))))
    }

    /// Insert value based on criteria in cmp function
    ///
    /// Returns the level at which the value is inserted
    pub fn insert(&mut self, data: T) -> Result<i32, Box<dyn std::error::Error>> {
        self.insert_replace(data)
    }

    /// Insert value based on criteria in cmp function
    ///
    /// Returns the level at which the value is inserted or replaced
    pub fn insert_replace(&mut self, data: T) -> Result<i32, Box<dyn std::error::Error>> {
        let mut height = 0;

        if let Some(ref mut value) = self.root {
            let (mut rc, compare_function) = (Rc::clone(value), self.cmp);
            'loo: loop {
                let rc1 = rc;
                let mut traverse = rc1.borrow_mut();

                match compare_function(&*traverse.data, &data) {
                    TreeInsOrder::Left => {
                        rc = if let Some(left) = &traverse.left {
                            Rc::clone(left)
                        } else {
                            traverse.left = Self::capsule(data);
                            height += 1;
                            self.len += 1;
                            break 'loo;
                        }
                    }
                    TreeInsOrder::Right => {
                        rc = if let Some(right) = &traverse.right {
                            Rc::clone(right)
                        } else {
                            traverse.right = Self::capsule(data);
                            height += 1;
                            self.len += 1;
                            break 'loo;
                        }
                    }
                    TreeInsOrder::Eq => {
                        // Replace the old value with the new value
                        traverse.data = Box::new(data.clone());
                        break 'loo;
                    }
                }
                height += 1;
            }
        } else {
            self.root = Self::capsule(data);
            self.len = 1;
        }

        Ok(height)
    }

    /// Assert data that is the root of the tree
    #[inline(always)]
    pub fn is_at_root(&mut self, data: T) -> bool {
        let some_func = self.cmp;
        match &self.root {
            Some(node) => some_func(node.borrow().data.as_ref(), &data) == TreeInsOrder::Eq,
            None => false,
        }
    }

    /// Returns whether function has child nodes with values as `left_data` in left node
    /// and `right_data` in right node
    #[inline(always)]
    pub fn has_both_children_with_compare(node: Link<T>, left_data: T, right_data: T) -> bool {
        node.borrow()
            .left
            .as_ref()
            .is_some_and(|f| *f.borrow_mut().data == left_data)
            && node
                .borrow()
                .right
                .as_ref()
                .is_some_and(|f| *f.borrow_mut().data == right_data)
    }

    /// Assert data that is the root of the tree
    #[inline(always)]
    pub fn get_root_node(&mut self) -> MaybeLink<T> {
        self.root.as_ref().map(Rc::clone)
    }

    /// Internal: Get left most children is a hierarchy given a parent node
    ///
    /// Returns the pair: shared `Rc` for `Node<T>` and it's parent
    fn get_last_node_with_parent(node: &Link<T>, parent: MaybeLink<T>) -> (Link<T>, MaybeLink<T>) {
        let (mut rc, mut par_rc) = (Rc::clone(node), parent);

        'iter: loop {
            let rc1 = match &rc.borrow().right {
                Some(right) => Rc::clone(right),
                None => break 'iter,
            };
            (par_rc, rc) = (Some(Rc::clone(&rc)), rc1);
        }
        (rc, par_rc)
    }

    /// Internal: Get left most children is a hierarchy given a parent node
    ///
    /// Returns the shared `Rc` for `Node<T>`
    fn get_first_node_with_parent(node: &Link<T>, parent: MaybeLink<T>) -> (Link<T>, MaybeLink<T>) {
        let (mut rc, mut par_rc) = (Rc::clone(node), parent);

        'iter: loop {
            let rc1 = match &rc.borrow().left {
                Some(left) => Rc::clone(left),
                None => break 'iter,
            };
            (par_rc, rc) = (Some(Rc::clone(&rc)), rc1);
        }
        (rc, par_rc)
    }

    /// Internal: finds a node that gives three values if the node is found.
    ///
    /// - The node with same value `data`
    /// - Parent of the searched node, if exists (else returns `None`)
    /// - Relation type between parent node and searched node: `Left`, `Right`
    /// otherwise returns `Eq` if there is no parent
    fn find_node(&mut self, data: &T) -> Option<(Link<T>, MaybeLink<T>, TreeInsOrder)> {
        let (mut parent, mut child_type) = (None, TreeInsOrder::Eq);
        let mut answer: Option<(_, _, _)> = None;

        if let Some(value) = &self.root {
            let (mut rc, compare_function) = (Rc::clone(value), self.cmp);
            'loo: loop {
                let rc1 = rc;
                let traverse = rc1.borrow();

                match compare_function(&*traverse.data, data) {
                    TreeInsOrder::Left => {
                        (rc, child_type) = (
                            match &traverse.left {
                                Some(left) => Rc::clone(left),
                                None => break 'loo,
                            },
                            TreeInsOrder::Left,
                        );
                    }
                    TreeInsOrder::Right => {
                        (rc, child_type) = (
                            match &traverse.right {
                                Some(right) => Rc::clone(right),
                                None => break 'loo,
                            },
                            TreeInsOrder::Right,
                        );
                    }
                    TreeInsOrder::Eq => {
                        answer = Some((Rc::clone(&rc1), parent, child_type));
                        break 'loo;
                    }
                }
                parent = Some(Rc::clone(&rc1));
            }
            answer
        } else {
            None
        }
    }

    /// Attach node to parent given the relation `child_type`
    fn attach(node: MaybeLink<T>, parent: Link<T>, child_type: TreeInsOrder) {
        match child_type {
            TreeInsOrder::Left => parent.borrow_mut().left = node,
            TreeInsOrder::Right => parent.borrow_mut().right = node,
            TreeInsOrder::Eq => {}
        }
    }

    /// Internal function: Deletes the node `to_delete`, a node other than root.
    ///
    /// Given context needed is a parent node `par` and relation between `par` and `to_delete`
    /// (Either `TreeInsOrder::Left` or `TreeInsOrder::Right`)
    ///
    /// On success, will delete the `to_delete` node.
    fn delete_inner_node(&mut self, to_delete: &Link<T>, par: Link<T>, child_type: TreeInsOrder) {
        let to_delete_node = to_delete.borrow();

        match (&to_delete_node.left, &to_delete_node.right) {
            // Deleted child does not have children, remove child of par
            (None, None) => Self::attach(None, par, child_type),
            // Deleted child have left node, attach left node as par's child
            (Some(left), None) => Self::attach(Some(Rc::clone(left)), par, child_type),
            // Deleted child have right node, attach left node as par's child
            (None, Some(right)) => Self::attach(Some(Rc::clone(right)), par, child_type),
            // Get last node of left subtree and attach to right
            (Some(left), Some(right)) => {
                let (right_most_node, parent) =
                    Self::get_last_node_with_parent(left, Some(Rc::clone(&par)));

                if let Some(new_par) = parent {
                    // If the parent of the right_most_node is the one we're
                    // deleting, then get the grand parent of right_most_node
                    // otherwise, get the direct parent to attach
                    if Rc::ptr_eq(to_delete, &new_par) {
                        Self::attach(
                            Some(Rc::clone(&right_most_node)),
                            Rc::clone(&par),
                            child_type,
                        );
                    } else {
                        new_par.borrow_mut().right = right_most_node.borrow_mut().left.clone();
                        Self::attach(
                            Some(Rc::clone(&right_most_node)),
                            Rc::clone(&par),
                            child_type,
                        );
                        right_most_node.borrow_mut().left = None;
                    }
                    right_most_node.borrow_mut().left = Some(Rc::clone(left));
                    right_most_node.borrow_mut().right = Some(Rc::clone(right));
                }
            }
        }
    }

    /// Internal function: deletes the node `to_delete`, a root node.
    fn delete_root(&mut self, to_delete: &Link<T>) {
        let to_delete_node = to_delete.borrow();
        match (&to_delete_node.left, &to_delete_node.right) {
            // Detach root node
            (None, None) => self.root = None,
            // Attach left node to current one
            (Some(left), None) => self.root = Some(Rc::clone(left)),
            // Attach right node to current one
            (None, Some(right)) => self.root = Some(Rc::clone(right)),
            // Take last node from left subtree
            (Some(left), Some(right)) => {
                let (last_node_from_left, parent_of_left) =
                    Self::get_last_node_with_parent(left, None);
                if Rc::ptr_eq(&last_node_from_left, left) {
                    self.root = Some(Rc::clone(&last_node_from_left));
                    last_node_from_left.borrow_mut().right = Some(Rc::clone(right));
                } else {
                    if let Some(last_node_left) = &last_node_from_left.borrow_mut().left {
                        if let Some(pol) = parent_of_left {
                            pol.borrow_mut().right = Some(Rc::clone(last_node_left));
                        }
                    } else if let Some(pol) = parent_of_left {
                        pol.borrow_mut().right = None;
                    }

                    last_node_from_left.borrow_mut().left = Some(Rc::clone(left));
                    last_node_from_left.borrow_mut().right = Some(Rc::clone(right));
                    self.root = Some(Rc::clone(&last_node_from_left));
                }
            }
        };
    }

    /// Delete from the binary search tree if that element exist
    ///
    /// ## Error
    /// Error is thrown as simple string when node is not found
    pub fn delete(&mut self, data: &T) -> Result<(), String> {
        let tree_info = self.find_node(data);

        if let Some((ref to_delete, parent, child_type)) = tree_info {
            // Parent implies found node is a child of `parent`.
            // If parent exists then node is not root
            if let Some(par) = parent {
                self.delete_inner_node(to_delete, par, child_type);
            } else {
                self.delete_root(to_delete);
            }

            to_delete.borrow_mut().left = None;
            to_delete.borrow_mut().right = None;
            self.len -= 1;

            Ok(())
        } else {
            Err("No node found".to_owned())
        }
    }

    /// Insert multiple elements at once.
    ///
    /// Returns the max height of the tree in return
    pub fn batch_insert(&mut self, data: &[T]) -> Result<i32, Box<dyn std::error::Error>> {
        Ok(data.iter().fold(0, |prev_height, t| {
            if let Ok(v) = self.insert((*t).clone()) {
                return if prev_height >= v { prev_height } else { v };
            }
            prev_height
        }))
    }

    /// Internal: Inorder traversal helper function.
    fn inorder_internal(node: &MaybeLink<T>, arr: &mut Vec<T>) {
        if let Some(exists) = &node {
            Self::inorder_internal(&exists.borrow().left, arr);
            arr.push(*exists.borrow().data.clone());
            Self::inorder_internal(&exists.borrow().right, arr);
        }
    }

    /// Returns an array of inorder traversed data.
    pub fn inorder(&mut self) -> Vec<T> {
        let mut internal_traversed = Vec::new();
        Self::inorder_internal(&self.root, &mut internal_traversed);
        internal_traversed
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_insert() {
        let mut p = BSTree::new(None);
        _ = p.batch_insert(&(0..200).collect::<Vec<u32>>().as_slice());
    }

    #[test]
    pub fn test_insert_replace() {
        type Pair<'a> = (i32, &'a str);
        let pair_cmp = |a: &Pair, b: &Pair| {
            if a.0 < b.0 {
                TreeInsOrder::Left
            } else if a.0 > b.0 {
                TreeInsOrder::Right
            } else {
                TreeInsOrder::Eq
            }
        };
        let mut p = BSTree::new(Some(pair_cmp));
        _ = p.batch_insert(&vec![
            (129, "some"),
            (234, "tree"),
            (34, "that i've"),
            (2, "constructed"),
            (3, "by certain"),
            (556, "custom"),
            (99, "criteria"),
        ]);

        _ = p.insert_replace((12, "new data"));
        _ = p.insert_replace((99, "some other thing"));
    }
    #[test]
    pub fn test_delete() {
        type Pair<'a> = (i32, &'a str);
        let pair_cmp = |a: &Pair, b: &Pair| {
            if a.0 > b.0 {
                TreeInsOrder::Left
            } else if a.0 < b.0 {
                TreeInsOrder::Right
            } else {
                TreeInsOrder::Eq
            }
        };
        let mut p = BSTree::new(Some(pair_cmp));
        _ = p.batch_insert(&vec![
            (129, "some"),
            (234, "tree"),
            (34, "that i've"),
            (2, "constructed"),
            (3, "by certain"),
            (556, "custom"),
            (99, "criteria"),
            (58, "that"),
            (226, "is made"),
            (335, "by"),
            (446, "pair_cmp"),
            (778, "as a"),
            (1077, "function"),
        ]);

        // Test insert replace and delete
        assert!(p.find_node(&(99, "criteria")) != None);
        assert!(p.find_node(&(12, "new data")) == None);

        _ = p.insert_replace((12, "new data"));
        _ = p.delete(&(99, "criteria"));

        // Test insert replace and delete
        assert!(p.find_node(&(99, "criteria")) == None);
        assert!(p.find_node(&(12, "new data")) != None);
        assert!(p.is_at_root((129, "some")));

        let len = p.len;
        _ = p.delete(&(129, "some"));

        assert!(!p.is_at_root((129, "some")));
        assert!(p.find_node(&(129, "some")).is_none());
        assert_eq!(p.inorder().len(), len - 1);

        // Check deleting node that has two children

        let some_node = p.find_node(&(556, "custom"));
        assert!(some_node.is_some_and(|(node, _, _)| { node.borrow().has_both_children() }));

        let len = p.len;
        _ = p.delete(&(556, "custom"));

        assert!(p.find_node(&(556, "custom")).is_none());
        assert_eq!(p.inorder().len(), len - 1);

        println!("{:?}", p.inorder());
    }

    #[test]
    pub fn test_delete_skewed() {
        let mut p = BSTree::new(None);
        let _ = p.batch_insert(&(0..10).collect::<Vec<u32>>().as_slice());

        assert!(p.find_node(&8) != None);
        assert!(p.inorder() == (0..10).collect::<Vec<u32>>());

        _ = p.delete(&8);

        assert!(p.find_node(&8) == None);
        assert!(p.inorder() == vec![0, 1, 2, 3, 4, 5, 6, 7, 9]);

        assert!(p.is_at_root(0));
        _ = p.delete(&0);

        assert!(p.is_at_root(1));
        assert!(p.find_node(&0) == None);
        assert!(p.inorder() == vec![1, 2, 3, 4, 5, 6, 7, 9]);

        _ = p.delete(&9);

        assert!(p.find_node(&9) == None);
        assert!(p.inorder() == vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    pub fn test_delete_shaped() {
        let mut p = BSTree::new(None);
        _ = p.batch_insert(&[5, 4, 3, 2, 1, 6, 7, 8, 9, 10]);
        assert!(p.is_at_root(5));

        _ = p.delete(&5);

        assert!(p.is_at_root(4));
        assert_eq!(p.inorder(), [1, 2, 3, 4, 6, 7, 8, 9, 10]);

        _ = p.delete(&9);

        assert!(p.find_node(&9) == None);
        let f = p.find_node(&10);

        assert!(f.is_some_and(|(node, par, _)| {
            *node.borrow().data == 10 && par.is_some_and(|p| *p.borrow().data == 8)
        }));

        assert_eq!(p.inorder(), [1, 2, 3, 4, 6, 7, 8, 10]);
    }

    #[test]
    pub fn test_delete_z_shaped() {
        let mut p = BSTree::new(None);
        // Tree shape expected to be
        //        10
        //       /
        //     5
        //      \
        //       8
        _ = p.batch_insert(&[10, 5, 8]);
        _ = p.delete(&5);

        assert_eq!(p.len, 2);
        assert_eq!(p.inorder(), [8, 10]);

        _ = p.batch_insert(&[15, 12]);
        assert_eq!(p.len, 4);

        _ = p.delete(&15);

        assert_eq!(p.len, 3);
        assert_eq!(p.inorder(), [8, 10, 12]);

        assert!(p.is_at_root(10));
        assert!(p.is_at_root(10));

        assert!(p
            .get_root_node()
            .is_some_and(|root| *root.borrow().data == 10
                && BSTree::has_both_children_with_compare(Rc::clone(&root), 8, 12)));
    }
}
