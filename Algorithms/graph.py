def depth_first_search(adj_matrix: dict, start_vertex):
    """
    A function for traversing a graph.
    The graph is represented as an adjacency matrix
    """
    visited = { key: False for key, value in adj_matrix.items() }
    
    def recursive_depth_first_search(matrix: dict, start_vertex: int):
        """
        A recursive function to traverse the graph 
        """
        visited[start_vertex] = True
        print(start_vertex)
        # perform some operations. Maybe printing
        for next_vertex in matrix[start_vertex]:
            if not visited[next_vertex]:
                recursive_depth_first_search(matrix, next_vertex)
    
    def iterative_depth_first_search(matrix: dict, start_vertex):
        """
        An iterative version of depth first search, uses 
        stack to visit the vertex and it's all connection before moving to 
        another neighbor.
        """
        stack = [start_vertex]
        while stack:
            vertex = stack.pop()
            # perform some operation
            print(vertex)
            visited[vertex] = True
            # Faster way for python:
            # stack.extend([next_vertex for next_vertex in matrix[start_vertex] if not visited[next_vertex]])
            for next_vertex in matrix[start_vertex]:
                if not visited[next_vertex]:
                    stack.append(next_vertex)

    return iterative_depth_first_search(adj_matrix, start_vertex)

def breadth_first_search(matrix: dict, start_vertex: int):
    """
    Level wise searching through the graph.
    Traverses the graph level-wise. Given a vertex, it's adjacent 
    vertex are all visited before traversing each of their children.
    """
    visited = { key: False for key, value in matrix.items() }

    def bfs(graph: dict | list[list], start_vertex: int | str) -> None:
        """
        Traverse the given graph in a level-wise fashion.
        """
        queue = [(start_vertex)]
        visited[start_vertex] = True
        answer = []
        while len(queue) > 0:
            current_vertex = queue[0]
            # Pops from front of list (this is an inefficient method)
            # since it causes items in list to shift.
            queue.pop(0)
            # Generally, vertex operations are done here.
            answer.append(current_vertex)

            for next_vertex in graph[current_vertex]:
                if not visited[next_vertex]:
                    visited[next_vertex] = True
                    # although the current shows only vertex info, we
                    # can put different info with it as well, depending 
                    # on user preferences.
                    queue.append(next_vertex)
        return answer

    return bfs(matrix, start_vertex)

def is_bipartite(graph: list[list[int]]) -> bool:
    """
    Checks whether the given graph is bi-partite
    i.e., two colorable graph
    Input is an adjacent graph.
    """
    n = len(graph)
    # Color array set for each of the vertices
    colors = [-1]*n
    for x in range(n):
        # If the vertex is not marked as visited
        if colors[x] == -1:
            # we traverse through vertex x and it's neighbors
            stack = [(x, 0)]
            while stack:
                v, c = stack.pop()
                # Assign the color to the vertex
                colors[v] = c
                # Neighbors should be marked as different colors
                for next_v in graph[v]:
                    # If not visited, then assign the next color
                    if colors[next_v] == -1:
                        stack.append([next_v, 1-c])
                    # If the color is the same as the current vertex, then
                    # this graph cannot be divided into two values.
                    # Return false.
                    elif colors[next_v] == c:
                        return False
    # If each vertex are colored, then
    # return true
    return True

def depth_first_search_with_combine(adj_matrix: list, start_vertex: int, combine, depth_first_condition, init):
    """
    Perform depth first search and combine the results as well.
    >>> 
    """
    visited = { key: False for key in adj_matrix.items() }
    result = { key: init for key in adj_matrix.items() }

    stack = [start_vertex]
    while stack:
        vertex = stack.pop()
        if visited[vertex]:
            for next_vertex in adj_matrix[vertex]:
                # Depending on the use case, user can either
                # combine result of current and neighboring vertex with the 
                # help of precomputed results stored in result, and can just 
                # return vertex if condition is not satisfied.
                result[vertex] = combine(vertex, next_vertex, result)
            
        else:
            visited[vertex] = True
            # Required. since to combine the results, user has to come
            # back to this vertex. So we push this current vertex first.
            stack.push(vertex)
            # and then add subsequent vertices next.
            stack.extend([next_vertex for next_vertex in adj_matrix[vertex] \
                            if depth_first_condition(vertex, next_vertex)])
    
    return result[start_vertex]

def kruskal_minimum_spanning_tree(edges_info: list, total_vertices: int):
    """
    Returns the minimum MST graphs
    >>> edges = [(0, 1, 10), (0, 2, 6), (0, 3, 5), (1, 3, 15), (2, 3, 4)]
    >>> kruskal_minimum_spanning_tree(edges, 4)
    (19, [(2, 3, 4), (0, 3, 5), (0, 1, 10)])
    >>> edges = [(1, 6, 10), (1, 2, 28), (2, 7, 14), (7, 5, 24), (7, 4, 18), (5, 4, 22), (3, 4, 12), (2, 3, 16), (5, 6, 25)]
    >>> kruskal_minimum_spanning_tree(edges, 7)
    (99, [(1, 6, 10), (3, 4, 12), (2, 7, 14), (2, 3, 16), (5, 4, 22), (5, 6, 25)])
    """
    def initialize(set_size: int) -> list:
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

    edges_info.sort(key=lambda x: x[2])
    parent = initialize(total_vertices)
    pairs, index, minimum_weight = [], 0, 0
    while index < len(edges_info) and len(pairs) < total_vertices:
        first_v, second_v, weight = edges_info[index]
        index += 1
        parent_first_v, parent_second_v = find_parent(parent, first_v), find_parent(parent, second_v)
        # Check whether the vertices have the same parent,
        # if they have, then we're avoiding linking, since, 
        # since they have a common parent, we're essentially closing the
        # endpoint to create a cycle, which is not possible since
        # this is a Tree (MST to be specific)
        if parent_first_v != parent_second_v:
            union(parent, parent_first_v, parent_second_v)
            pairs.append((first_v, second_v, weight))
            minimum_weight += weight

    if len(pairs) != total_vertices - 1:
        return None, None
    return minimum_weight, pairs

def prims_algorithm(adj_matrix: list, start_vertex: int, total_vertices: int):
    """
    Finds the MST by using Prim's method.
    Uses the adjacency matrix and heap to construct MST
    >>> adj = [[(1, 10), (2, 6), (3, 5)], [(0, 10), (3, 15)], [(0, 6), (3, 4)], [(0, 5), (2, 4)]]
    >>> prims_algorithm(adj, 0, 4)
    (19, [(0, 3, 5), (3, 2, 4), (0, 1, 10)])
    >>> # edges = [(1, 6, 10), (1, 2, 28), (2, 7, 14), (7, 5, 24), (7, 4, 18), (5, 4, 22), (3, 4, 12), (2, 3, 16), (5, 6, 25)]
    >>> adj = [[], [(6, 10), (2, 28)], [(7, 14), (1, 28), (3, 16)], [(4, 12), (2, 16)], [(7, 18), (3, 12)], [(7, 24), (4, 22), (6, 25)], [(1, 10), (5, 25)], [(2, 14), (5, 24), (4, 18)]]
    >>> prims_algorithm(adj, 1, 8)
    (99, [(1, 6, 10), (6, 5, 25), (5, 4, 22), (4, 3, 12), (3, 2, 16), (2, 7, 14)])
    """
    class heap:
        """
        Generalize binary heap, arranges elements according to the 
        comparison operator, by default initializes max heap.
        """
        def __init__(self, comparison_operator=None):
            self.heap_array = []
            if comparison_operator is not None:
                self.comparison_operator = comparison_operator
            else:
                # default max heap
                self.comparison_operator = lambda root, child: root >= child

        def push(self, item):
            """
            Pushes an element `item` into the heap.
            """
            # subsituting references for high performance
            ha, co = self.heap_array, self.comparison_operator
            ha.append(item)
            last_index = len(ha) - 1
            # rearrange the added element such the parent-children condition should be satisfied.
            # so swap until the parent-child is not satisfied.
            while last_index and not(co(ha[(last_index - 1) // 2], ha[last_index])):
                ha[last_index], ha[(last_index - 1)//2] = ha[(last_index - 1)//2], ha[last_index]
                last_index = (last_index - 1) // 2

        def heapify(self):
            """
            Used after popping off element from the heap.
            Rearrangement is done when a last element in the heap is put at the 
            top of the heap.

            Swapping is done in this stage, until the heap stage is satisfied:
            - A child is selected that can be the parent of root and another sibling
            - Swap these elements.
            - Repeat this process till such child is not found, or the root element follows
            condition.
            """
            i = 0
            # substitution for better performance
            ha, co = self.heap_array, self.comparison_operator
            # Follow this condition till the last of heap.
            while 2*i + 1 < len(ha):
                left, right = 2 * i + 1, 2 * i + 2
                # if there is no right child.
                if right == len(ha):
                    if not(co(ha[i], ha[left])):
                        ha[i], ha[left] = ha[left], ha[i]
                        i = left
                    else:
                        break
                elif right < len(ha):
                    # check whether 'left' is correct for both root and
                    # sibling
                    if co(ha[left], ha[right]):
                        if not(co(ha[i], ha[left])):
                            ha[i], ha[left] = ha[left], ha[i]
                            i = left
                        else:
                            break
                    elif not(co(ha[i], ha[right])):
                        ha[i], ha[right] = ha[right], ha[i]
                        i = right
                    else:
                        break
                else:
                    break

        def pop(self):
            """
            Pop element from heap: rearranges all element according to the comparator 
            """
            if len(self.heap_array) == 0:
                return None
            self.heap_array[0], self.heap_array[-1] = self.heap_array[-1], self.heap_array[0]
            item_to_return = self.heap_array.pop()
            self.heapify()
            return item_to_return

        def top(self):
            """
            Returns the top of the heap array.
            returns none if heap array is empty
            """
            if self.heap_array:
                return self.heap_array[0]
            return None

        def __len__(self):
            """
            Length of heap array
            """
            return len(self.heap_array)

        def __str__(self):
            """
            Print heap
            """
            return '<heap> %s ' % str(self.heap_array)

    pq = heap(lambda parent, child: parent[2] < child[2])

    pq.push((-1, start_vertex, 0))
    total_cost = 0
    visited_vertices = set()
    pairs = []
    while len(pq) > 0 and len(pairs) < total_vertices - 1:
        prev_vertex, vertex, cost = pq.pop()
        # There are chances that a vertex is already visited, if it did, then a cycle
        # is formed. Just prevent it.
        if vertex not in visited_vertices:
            total_cost += cost
            # visit the vertex
            visited_vertices.add(vertex)
        
            if prev_vertex != -1:
                pairs.append((prev_vertex, vertex, cost))
        
            # Insert the adjacent values in priority queue
            for next_vertex, next_cost in adj_matrix[vertex]:
                if next_vertex not in visited_vertices:
                    pq.push((vertex, next_vertex, next_cost))
    
    return total_cost, pairs

def topological_sorting(adj_matrix: list) -> list:
    """
    Performs topological sorting on directed graph.
    """
    def depth_first_search_sorting(vertex: int, visited: dict, adj_matrix: list, sorted_vertex: list, cycle_check: set):
        """
        Performs depth first search on directed graph, and sort the vertices by visiting all the
        incoming edges: this is to remove and sort the vertices that are incoming

        If there is a cycle, then sorting is not possible, return None, else sort the vertices and
        after sorting, insert the current vertex.
        """
        if vertex in cycle_check:
            return None
        visited[vertex] = True
        cycle_check.add(vertex)
        for next_vertex in adj_matrix[vertex]:
            if not visited[next_vertex]:
                if depth_first_search_sorting(next_vertex, visited, adj_matrix, sorted_vertex, cycle_check) is None:
                    return None

        sorted_vertex.append(vertex)
        return True

    visited = { key: False for key in range(len(adj_matrix)) }

    sorted_vertex = []
    for vertex in range(len(adj_matrix)):
        if not visited[vertex]:
            cycle_check = set()
            if depth_first_search_sorting(vertex, visited, adj_matrix, sorted_vertex, cycle_check) is None:
                return None

    sorted_vertex = reversed(sorted_vertex)
    return sorted_vertex

if __name__ == '__main__':
    from doctest import testmod
    testmod()
