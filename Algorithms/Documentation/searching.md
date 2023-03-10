# Searching.
In CS, a process of efficiently traversing a data structure for finding certain value/thing.
Searching can be done on:
- [[arrays_1d|Arrays]]
- [[linked_list|Linked List]]
- Trees
- [[graphs_1]]
- [[string|Strings]]
- [[prefix_tree#Trie Prefix tree|Prefix Trees]]
- Suffix trees, etc.

## Search space (limits)
- For an array:
	- Searching an element in an array would be the length of an array. 

Linear searching takes $O(n)$ time and $O(1)$ space.

- Search space could also be a domain range (e.g., $x\in[1,n)$ depending on type of problem).

# Searching Type
## Linear Search

Scanning through elements one after the other (in general moving to adjacent ones after unsuccessful find).

```python
def linear_search(array: list, target: int) -> int:
	"""
	Searches the value in a list. Takes O(n) time.
	"""
	for index, value in enumerate(array):
		if value == target:
			return index
	return -1
```

Finding $\lfloor\sqrt{n}\rfloor$:

```python
def isqrt_linear_search(target: int) -> int:
	"""
	Searches the sqrt in a list. Takes O(target) time.
	"""
	for value in range(target+1):
		if value * value == target:
			return value
		elif value * value > target:
			return value-1
	return target
```

## Binary Search

The constraints with the binary search is that the search space should be in a sorted order.

Binary search on space $S$ in a given frame $L$ and $R$ to find value $T$ is given as:

$$
\text{b\_search}(S,L,R,T,f)=\left\{
\begin{array}{cl}
-1,&L\geq R\\
M,&M=\dfrac{L+R}{2}, f(S,M,T)=0\\
\text{b\_search}(S,M+1,R,T),&M=\dfrac{L+R}{2},f(S,M,T)<0\\
\text{b\_search}(S,L,M-1,T),&M=\dfrac{L+R}{2},f(S,M,T)>0\\
\end{array}
\right.
$$

Finding $\lfloor\sqrt{n}\rfloor$ of a number $n$.

```python
def sqrt_bin_search(n: int) -> int:
    """
    Evaluates integer square root by using binary search,
    therefore, takes `O(n)` time.
    >>> sqrt_bin_search(25)
    5
    >>> sqrt_bin_search(100)
    10
    >>> sqrt_bin_search(65)
    8
    """
    if n == 1: return 1
    low, high = 0, n
    while low < high:
        mid = (low + high) // 2
        if mid * mid < n:
            low = mid + 1
        elif mid * mid > n:
            high = mid - 1
        else:
            return mid
    return low
```

There is even faster way to find [[intermediate_maths#Square root of a number sqrt n|square root of any number]].

# [[graphs_1|Graphs]].
In graphs, we use Depth First Search (DFS) and Breadth First Search (BFS) for seaeching within the graphs.

## Search space
The search space for the graph is $V$ or/and $E$ depending on the approach/task at hand.

## Depth first Search.
Type of search in which from a starting vertex, a neighbor is inspected one-by-one immediately, if they are not visited.

```python
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
```

Visiting each vertex takes $O(|V|+|E|)$, with an extra space of $O(|V|)$ (max-stack size and visited vertex marking).

## Breadth First Search
Breadth First Search (BFS), intends to seek all the neighbors first and visits them one by one. It acheives this by putting all the nodes in a [[queue|queue]].

```python
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
```

Visiting each vertex takes $O(|V|+|E|)$ (checking each edges and visiting vertex) requires that amount of time, while queue and visited vertex takes $O(|V|)$ space.

# Substring Searching
There are many methods to search a pattern $T$ in a string $S$, some of them include:
- [[string##Rabin Karp method|Rabin Karp Method]]: $O(|S|+|T|)$
- [[string#Knuth-Morris-Pratt KMP Method|Knuth-Morris-Pratt Method]]: $O(|S|+|T|)$
- 