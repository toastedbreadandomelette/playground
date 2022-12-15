
def intialize(set_size: int) -> list:
    """
    Initialize the parent vertex to itself.

    e.g.,
    ```
    [1]     [2]     [3]     [4]

    parent_list = [0, 1, 2, 3, 4]
    ```
    """
    return [ vertex for vertex in range(set_size + 1) ]

def find_parent(parent_list: list, vertex: int) -> int:
    """
    Find the parent of the current vertex, that are linked via
    union function or are already linked.

    ```
    e.g.,
    [1]         [4]
     |
    [2]
     |
    [3]

    => parent_list = [0, 1, 1, 2, 4]
    ```

    - find_parent(parent_list, 3) will modify the parent_list to

    ```
    [1] - [3]        [4]
     |
    [2]

    => parent_list = [0, 1, 1, 1, 4]
    ```
    >>> find_parent([0, 1, 1, 2, 4], 3)
    1
    """
    if parent_list[vertex] == vertex:
        return vertex
    parent_list[vertex] = find_parent(parent_list, parent_list[vertex])
    return parent_list[vertex]

def union(parent_list: list, first_vertex: int, second_vertex: int) -> None:
    """
    Find the disjoint sets and join their parents.

    ```
    e.g.,
    [1] - [3]         [4] - [5] - [6] 
     |
    [2]

    => parent_list = [0, 1, 1, 2, 4, 4, 5]
    ```

    - union(parent_list, 3, 6) will modify the parent_list to

    ```
              [4] - [6]
             /   \\
    [3] - [1]     [5]
           |
          [2]

    => parent_list = [0, 4, 1, 1, 4, 4, 4]
    ```
    >>> parent_list = [0, 1, 1, 2, 4, 4, 5]; union(parent_list, 3, 6); parent_list
    [0, 4, 1, 1, 4, 4, 4]
    """
    first_vertex, second_vertex = find_parent(parent_list, first_vertex), find_parent(parent_list, second_vertex)
    if first_vertex != second_vertex:
        parent_list[first_vertex] = parent_list[second_vertex]

if __name__ == "__main__":
    from doctest import testmod
    testmod()
    