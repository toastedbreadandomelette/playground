/// Tree Insertion Order to instruct how to traverse binary search tree
#[derive(Debug, Eq, PartialEq)]
pub enum TreeInsOrder {
    /// Instruct algorithm to operate on left
    Left,
    /// Instruct algorithm to operate on right
    Right,
    /// Instruct algorithm to operate when node contains current value
    Eq,
}
