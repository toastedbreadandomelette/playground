use std::marker::PhantomData;

#[derive(PartialEq, Clone, Debug)]
pub struct Node<'a, T> where T: Copy + std::fmt::Debug {
    // Actual data to store
    data: T,
    // Stores height difference between child nodes
    diff_height: i16,
    // Phantom Data
    ph: PhantomData<&'a T>,
    // Left link
    left: Option<Box<Node<'a, T>>>,
    // Right link
    right: Option<Box<Node<'a, T>>>,
    // Parent link
    parent: Option<Box<Node<'a, T>>>
}

impl<'a, T> Node<'a, T> where T: Copy + std::fmt::Debug {
    pub fn new(data: T) -> Self {
        Node {
            data,
            diff_height: 0,
            ph: PhantomData,
            left: None,
            right: None,
            parent: None,
        }
    }

    /// Compute height difference between two branches.
    /// 
    /// This updates all the values of height differences in all tree
    /// branches
    pub fn height_difference_compute(&mut self) -> i16 {
        let diff_height = match (
            self.left.as_deref_mut(), 
            self.right.as_deref_mut()
        ) {
            (None, None) => 1,
            (Some(left_node), None) => {
                1 + left_node.height_difference_compute()
            },
            (None, Some(right_node)) => {
                -(1 + right_node.height_difference_compute())
            },
            (Some(left_node), Some(right_node)) => {
                left_node.height_difference_compute() - right_node.height_difference_compute()
            }
        };
        self.diff_height = diff_height;
        diff_height
    }

    /// Update height difference with respect to the current
    /// node
    pub fn update_height(&mut self) -> i16 {
        let diff_height = match (
            self.left.as_deref_mut(), 
            self.right.as_deref_mut()
        ) {
            (None, None) => 1,
            (Some(left_node), None) => {
                1 + left_node.diff_height
            },
            (None, Some(right_node)) => {
                -(1 + right_node.diff_height)
            },
            (Some(left_node), Some(right_node)) => {
                left_node.diff_height - right_node.diff_height
            }
        };
        self.diff_height = diff_height;
        diff_height
    }

    #[inline]
    pub fn boxed(data: T) -> Box<Self> {
        Box::new(Node::new(data))
    }
}

/// Maybe add comparison node
#[derive(PartialEq, Debug)]
pub struct Tree<'a, T>(
    Option<Node<'a, T>>,
    fn(T, T) -> bool
) where T: PartialOrd + Copy + std::fmt::Debug;

impl <'a, T> Tree<'a, T>
where 
    T: PartialOrd + 
       Copy + 
       std::fmt::Display +
       std::fmt::Debug
{
    /// Create a new instance of new tree.
    /// 
    /// Creates a tuple of root node and a comparator
    #[inline]
    pub fn new(comp: Option<fn(T, T) -> bool>) -> Self {
        match comp {
            Some(left_add_comparator) => Self(None, left_add_comparator),
            None => Self(None, |a, b| a > b)
        }
    }

    /// Add an item in a tree node
    /// 
    /// Todo: It should also update the tree heights if
    /// absolute height difference is more than 2.
    pub fn add(&mut self, data: T) {
        let mut parent_history: Vec<Node<'a, T>> = Vec::new();
        match self.0 {
            Some(ref mut root) => {
                // Traversal node
                let mut traversal_node: &mut Node<T> = root;
                loop {
                    // A comparator that adds to left
                    if self.1(traversal_node.data, data) {
                        // Add to the left as a link if node does not
                        // exist and exit otherwise go to the link
                        if None == traversal_node.left {
                            traversal_node.left = Some(Node::boxed(data));
                            break;
                        } else {
                            // let len = parent_history.len();
                            parent_history.push(traversal_node.clone());
                            traversal_node = traversal_node
                                .left
                                .as_deref_mut()
                                .unwrap();
                        }
                    } else {
                        // Add to the right as a link if node does not
                        // exist and exit otherwise go to the link
                        if None == traversal_node.right {
                            traversal_node.right = Some(Node::boxed(data));
                            break;
                        } else {
                            parent_history.push(traversal_node.clone());
                            traversal_node = traversal_node
                                .right
                                .as_deref_mut()
                                .unwrap();
                        }
                    }
                }
            }
            None => {
                self.0 = Some(Node::new(data));
            }
        }
    }

    #[inline]
    pub fn dbg(&self) {
        if let Some(root) = &self.0 {
            Self::print_all(Some(root));
        }
    }

    #[inline]
    pub fn print(&self) {
        if let Some(root) = &self.0 {
            Self::print_all(Some(root));
        }
    }

    #[inline]
    pub fn yield_all(&self) -> Vec<T> {
        if let Some(root) = &self.0 {
            Self::yield_all_internal(Some(root))
        } else {
            vec![]
        }
    }

    pub fn yield_all_internal(node: Option<&Node<'a, T>>) -> Vec<T> {
        if let Some(nd) = node {
            let mut left_nodes_list = Self::yield_all_internal(nd.left.as_deref());
            left_nodes_list.push(nd.data);
            vec![
                left_nodes_list,
                Self::yield_all_internal(nd.right.as_deref())
            ]
                .into_iter()
                .flatten()
                .collect::<Vec<T>>()
        } else {
            vec![]
        }
    }

    pub fn dbg_all(node: Option<&Node<'a, T>>) {
        if let Some(nd) = node {
            Self::print_all(nd.left.as_deref());
            println!("{:?}", nd);
            Self::print_all(nd.right.as_deref());
        }
    }

    pub fn print_all(node: Option<&Node<'a, T>>) {
        if let Some(nd) = node {
            Self::print_all(nd.left.as_deref());
            println!("{}", nd.data);
            Self::print_all(nd.right.as_deref());
        }
    }
}

#[test]
pub fn test_avl() {
    let mut tree: Tree<u32> = Tree::new(None);
    [10, 20, 30, 15, 40, 35, 33, 5].iter().for_each(|item| tree.add(*item));
    assert_eq!(tree.yield_all(), [5, 10, 15, 20, 30, 33, 35, 40]);
}

pub fn main () {
    let mut tree: Tree<u32> = Tree::new(None);
    [10, 20, 30, 15, 40, 35, 33, 5].iter().for_each(|item| tree.add(*item));
    tree.print();
}
