class tree_node:
    """
    A tree node that contains data, and references to
    two other similar nodes, by left and right value
    """
    def __init__(self, data=None):
        self.data, self.left, self.right = data, None, None

class binary_search_tree:
    """
    Simple utils for binary search tree. Value to the 
    left child is smaller than parent value and value 
    to the right child is greater.
    ```
    e.g.,         5
               /     \\
             2         8
           /         /   \\
          1         6     10
    ```
    """
    def __init__(self):
        self.root_node = None

    def __iadd__(self, data: any):
        """
        Adds the value to the tree node.
        
        Traverses till it gets the right node to attach to
        """
        if self.root_node is None:
            self.root_node = tree_node(data)
            return self
        traverse = self.root_node
        while True:
            if data > traverse.data:
                if traverse.right is None:
                    traverse.right = tree_node(data)
                    break
                else:
                    traverse = traverse.right
            elif data < traverse.data:
                if traverse.left is None:
                    traverse.left = tree_node(data)
                    break
                else:
                    traverse = traverse.left
            else:
                break
        return self
    
    def tree_contains(self, data: any):
        """
        Check whether the tree contains node holding data
        `data`
        :returns node if contains the exact data, else None 
        """
        if self.root_node is None:
            return None
        traverse = self.root_node
        while True:
            if data > traverse.data:
                if traverse.right is None:
                    return None
                else:
                    traverse = traverse.right
            elif data < traverse.data:
                if traverse.left is None:
                    return None
                else:
                    traverse = traverse.left
            else:
                return traverse

    def __contains__(self, data: any) -> bool:
        """
        Check whether the tree contains the data,
        returns true or false
        """
        return self.tree_contains(data) is not None

    def delete_node(self, data: any) -> None:
        """
        Finds and deletes the node
        # Note: This might not be the smallest value but it's 
        # simple to understand due to case handling .
        """
        if self.root_node is None:
            return None
        traverse, parent = self.root_node, None
        while traverse is not None:
            if data > traverse.data:
                parent = traverse
                traverse = traverse.right
            elif data < traverse.data:
                parent = traverse
                traverse = traverse.left
            else:
                break
        if traverse is None:
            return None
        # If the node to delete is the root node.
        if traverse == self.root_node:
            if traverse.left is not None and traverse.right is None:
                self.root_node = traverse.left
                traverse.left = None
            elif traverse.left is None and traverse.right is not None:
                self.root_node = traverse.right
                traverse.right = None
            elif traverse.left is not None and traverse.right is not None:
                self.root_node = traverse.left
                child_of_del_node = traverse.left
                while child_of_del_node.right is not None:
                    child_of_del_node = child_of_del_node.right
                child_of_del_node.right = traverse.right
                traverse.left = traverse.right = None
        else:
            if traverse.left is None and traverse.right is None:
                if parent.left == traverse:
                    parent.left = None
                else:
                    parent.right = None
            elif traverse.left is None and traverse.right is not None:
                if parent.left == traverse:
                    parent.left = traverse.right
                else:
                    parent.right = traverse.right
                traverse.right = None
            elif traverse.left is not None and traverse.right is None:
                if parent.left == traverse:
                    parent.left = traverse.left
                else:
                    parent.right = traverse.left
                traverse.left = None
            else:
                if parent.left == traverse:
                    parent.left = traverse.left
                else:
                    parent.right = traverse.left
                # attach right child to traverse.left => rightmost children
                child_of_del_node = traverse.left
                while child_of_del_node.right is not None:
                    child_of_del_node = child_of_del_node.right
                child_of_del_node.right = traverse.right
                traverse.left = traverse.right = None
        # delete the node.
        del traverse

    def max_height(self, node) -> int:
        """
        Evaluate max depth of a tree. This recursively calculates the
        height of a binary tree (does not need to be a search tree as well,
        as long as the tree is valid).
        Below generated tree.
                     5
                3         7
              2   4     6   8
            1                 10
                                 12
        >>> tr = binary_search_tree()
        >>> for val in [5, 3, 4, 2, 1, 7 ,6 ,8, 10, 12]:  tr += val
        >>> tr.max_height(tr.root_node)
        5
        """
        if node is not None:
            return 1 + max(self.max_height(node.left), self.max_height(node.right))
        return 0

    def max_diameter(self) -> int:
        """
        Returns the max length from one leaf node
        to a different leaf node.
        Note: this function considers root having only one child as 
        a leaf as well, (i.e., nodes having total degree(indeg+outdeg)=1).
        First generated tree.
                     5
                3         7
              2   4     6   8
            1                 10
                                 12

        Second generated tree
        >>> tr = binary_search_tree()
        >>> for val in [5, 3, 4, 2, 1, 7 ,6 ,8, 10, 12]:  tr += val
        >>> tr.max_diameter()
        7
        """
        def helper(node, isroot=True) -> tuple:
            """
            Helper function to evaluate the longest diameter of the given tree
            T.
            """
            if node is not None:
                left_max_diameter, left_max_depth = helper(node.left, False)
                right_max_diameter, right_max_depth = helper(node.right, False)
                # Case where a node has at most one child, then it's
                # not possible to create a path, except for root,
                # which is open-ended at one side which in this case 
                # considered as a leaf node.
                if not isroot and (node.left is None or node.right is None): 
                    return 0, max(left_max_depth, right_max_depth) + 1

                curr_max_diameter = max(left_max_diameter, right_max_diameter, left_max_depth + right_max_depth + 1)
                curr_max_depth = max(left_max_depth, right_max_depth) + 1
                return curr_max_diameter, curr_max_depth
            return 0, 0

        max_d, max_dpth = helper(self.root_node)
        return max_d - 1



def inorder(root_node):
    """
    Inorder traversal of a binary tree
    """
    if root_node is not None:
        inorder(root_node.left)
        print(root_node.data)
        inorder(root_node.right)

def preorder(root_node):
    """
    Preorder traversal of a binary tree
    """
    if root_node is not None:
        print(root_node.data)
        preorder(root_node.left)
        preorder(root_node.right)

def postorder(root_node):
    """
    Postorder traversal of a binary tree
    """
    if root_node is not None:
        postorder(root_node.left)
        postorder(root_node.right)
        print(root_node.data)

if __name__ == "__main__":
    tr = binary_search_tree()
    tr += 5
    tr += 3
    tr += 4
    tr += 2
    tr += 7
    tr += 6
    tr += 8
    inorder(tr.root_node)
    print(8 in tr, 7 in tr, 1 in tr)
    tr.delete_node(8)
    inorder(tr.root_node)
    print()
    tr.delete_node(4)
    inorder(tr.root_node)
    tr.delete_node(5)
    print()
    inorder(tr.root_node)
