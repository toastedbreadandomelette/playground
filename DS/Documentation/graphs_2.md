# Graph Part 2
[[graphs_1|Part 1 is here]].

# Topological Sorting of graph $G$.
Topological sorting is a way to sort the vertices $v$ based on the incoming edges $e_v$. To be clear, if the incoming edges on vertex $v$ is zero, then $v$ is removed from $V$.

The pre-requisite is the graph $G$ should be DAG (Directed Acyclic Graph). $\implies$ If the cycles exists or the graph is undirected, then the solution for the program does not exists.

### Explaination: 
$\forall\ v\in V$, we perform the following operations (say $\text{dfs}(v,V_s,C=\{\})$):
	- If $v\in C$ then we exit. 
	- otherwise, we put $v$ into visited set $V_s$ and cycle set $C$
	- For each neighbor of vertex $v$ (say $nv$)
		- we perform $\text{dfs}(nv,V_s,C)$ if $nv\notin V_s$.
		- if the $\text{dfs}$ is exited, we do not explore since there would be no way to sort these vertices.

```python
def topological_sorting(adj_matrix: list) -> list:
    """
    Performs topological sorting on directed graph.
    """
    def depth_first_search_sorting(vertex: int, visited: dict, adj_matrix: list, sorted_vertex: list, cycle_check: set):
        """
        Performs depth first search on directed graph.
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

    visited = { key: False for key in range(len(adj_matrix)) }

    sorted_vertex = []
    for vertex in range(len(adj_matrix)):
        if not visited[vertex]:
            cycle_check = set()
            if depth_first_search_sorting(vertex, visited, adj_matrix, sorted_vertex, cycle_check) is None:
                return None

    sorted_vertex = reversed(sorted_vertex)
    return sorted_vertex
```

Complexity: Since we perform DFS, time complexity is $O(|V|+|E|)$, whereas space complexity is $O(|V|)$ (visited array, and recursive calls takes $|V|$ space).

## Applications
- Solving dependencies across various schemes:
	- Compilers: Most of them are included in various files, except some of them are independent. This algorithm resolves the dependencies in the files while compilation. 
	- [An hypothetical example](https://leetcode.com/problems/course-schedule-ii/).

# Existence of bridges in Graph.
## Offline method
We solve whether bridge exists using DFS. This will have time complexity $O(|V|+|E|)$. 

A bridge is defined as edge $(u,v)\in E$ such that removing such edge would result in two new subgraphs. 
OR
A bridge is defined as edge $(u,v)\in E$ such that there is no edge from $v$ or it's other descendants to $u$ or it's ancestors.

To evaluate the bridges, we traverse the graph using unit time $t$ (as a jump from $u$ to $v$). 

Let us denote $Tin_u$ as in time for vertex $u$. So by visiting each vertex, we increase the timer $t:=t+1$.

With this, we'll introduce $L_u$ as:

$$
L_u=\min\left\{
\begin{array}{cl}
Tin_u,\\
Tin_p,&\forall\ p\in V,(u,p)\in E\ \wedge\ ((u\rightarrow p)\text{ is a back-edge})\\
L_p,&\forall\ p\in V,\ (u,p)\in E,\ \wedge\ (u\rightarrow p\text{ is a tree edge} )
\end{array}
\right.
$$

Considering that there is back edge: i.e., an edge from descendants of $p$ to ancestors of $u$ (or $u$ itself). In that case, $L_p\leq Tin_u$. Otherwise the edge $u\rightarrow p$ is a bridge.

```cpp
/**
 * @brief Returns all the bridges in the graph as a list of edges.
 * This is an offline algorithm that determines all the bridges in the 
 * graph. If any edges/vertices is added in the graph, then we need to
 * compute the whole graph again.
 * 
 * @param adj The graph input
 * @param start_vertex start vertex for the traversal
 * @return vector<pair<int, int>> pair of vertices denoting the bridge
 */
vector<pair<int, int>> find_all_bridges(vector<vector<int>>&adj, int start_vertex) {
    vector<bool>visited(adj.size(), false);
    using edge = pair<int, int>;
    vector<edge> result;

    // Declaring low: lowest point of visiting time.
    vector<int> low(adj.size(), 0);
    // tin: visiting time for vertex v
    vector<int> tin(adj.size(), 0);

    int timer = 0;
    using func_blueprint = function<vector<edge>(vector<bool>&, const vector<vector<int>>&, int, int)>;
    
    /**
     * @brief Find bridges in a subgraph G
     * 
     * @param visited visited vertices
     * @param adj adjacency graph
     * @param curr_vertex currently processing vertex
     * @param start_vertex previous visited vertex
     * @returns vector<pair<int, int>> pair of integers representing edges that are
     * bridges
     */
    func_blueprint find_bridges = 
        [&find_bridges, &low, &tin, &timer](vector<bool>&visited, const vector<vector<int>>& adj, int curr_vertex, int prev_vertex=-1) {
        visited[curr_vertex] = true;
        vector<edge>bridges;
        // Visit the current vertex
        low[curr_vertex] = tin[curr_vertex] = timer++;
        
        // Loop on adjacent vertices
        for (auto &next_vertex: adj[curr_vertex]) {
            // We need to make sure that the vertex we're coming back to
            // is not immediate vertex, so this check.
            if (next_vertex != prev_vertex) {
                // If not visited, visit it and check for the possible bridges in the 
                // subgraphs.
                if (!visited[next_vertex]) {
                    auto next_bridges = find_bridges(visited, adj, next_vertex, curr_vertex);
                    bridges.insert(bridges.end(), next_bridges.begin(), next_bridges.end()); 
                    low[curr_vertex] = min(low[curr_vertex], low[next_vertex]);

                    // If there is no back (returning) edge to current vertex, the value 
                    // low[next_vertex] will be higher than the low[curr_vertex]. If there was one, then
                    // The next_vertex would have been marked with the lowest in time (in else part)
                    if (low[next_vertex] > low[curr_vertex]) {
                        bridges.push_back({curr_vertex, next_vertex});
                    }
                } else {
                    low[curr_vertex] = min(low[curr_vertex], tin[next_vertex]);
                }
            }
        }
        return bridges;
    };

    for (int i = 0; i < adj.size(); ++i) {
        if (!visited[i]) {
            auto sub_graph_bridges = find_bridges(visited, adj, i, -1);
            for (auto &x: sub_graph_bridges) {
                result.push_back(x);
            }
        }
    }

    return result;
}
```

## Online method

# Shortest Path Algorithm
## Floyd-Warshall's algorithm
Floyd Warshall's shortest path algorithm is a simple algorithm that finds shortest paths between every vertices all at once in $O(n^3)$ complexity.

Let us consider [[graphs_1#Matrix|2D matrix]] as input (say $M$) of size $n\times n$. Then, the shortest path from $i\rightarrow j$ (we'll denote by $S_{ij}$) is done by:

$$
\forall\ (i,j)\in[1,n], S_{ij}=\min\limits_{\begin{matrix}k=1\\(i,k)\in E,\ (k,j)\in E\end{matrix}}^{n}(S_{ik}+S_{kj})
$$

We can see why the complexity is $O(n^3)$. We work on every pair $i,j$ and we check closest distance for every existing path from $i\rightarrow j$ via every possible intermediate vertex $k$ (so the route would be $i\rightarrow \cdots \rightarrow k\rightarrow\cdots\rightarrow j$).

## Dijkstra's Shortest Path Algorithm
Dijkstra's Algorithm is an algorithm for finding shortest path on a graph $G$ from a source vertex $u$ to destination vertex $v$. The shortest path is evaluated from a minimum cost $w$. 

When $\forall\ e=(u,v,w)\in E,w=1$, [[searching#Breadth First Search|Breadth First Search]] is a preferred way to evaluate the shortest path. Otherwise we need a different way to sort out the shortest value from a set of weighted edges. If searching is preferred, the complexity goes to $O(|V|^2)$. Otherwise, we use [[heaps|min-heap]] to retrieve the minimum value in $O(\log_2{n})$ time complexity.

```cpp
/**
 * @brief comparator for the min-priority queue. 
 * 
 */
struct comparator {
    bool operator()(const tuple<int, int, int> &a, const tuple<int, int, int> &b) {
        return get<2>(a) > get<2>(b);
    };
};

/**
 * @brief Evaluate the shortest path from source vertex to destination vertex
 * 
 * @param adj_graph graph G as a reference.
 * @param source_vertex starting vertex
 * @param dest_vertex the destination vertex
 * @return pair<int, vector<int>> integer denoting the cost and vector of 
 * path from source_Vertex to dest_vertex
 */
pair<int, vector<int>> dijkstra_shortest_path(vector<vector<pair<int, int>>>&adj_graph, int source_vertex, int dest_vertex) {
    using edge = tuple<int, int, int>;
    // using min-priority queue for evaluating the shortest path
    priority_queue<edge, vector<edge>, comparator> pq;
    pq.push(make_tuple(-1, source_vertex, 0));
    
    vector<int>parent(adj_graph.size() + 1, 0);
    
    // Initialize the parents as itself, so that we find the source and 
    // destination by following the path
    for (auto i = 1; i < adj_graph.size(); ++i) {
        parent[i] = i;
    }

    vector<int>min_path;
    vector<bool>visited(adj_graph.size() + 1, false);
    // while pq is not empty, we perform the following operation.
    // this is not efficient when there are more edges, then it's 
    // essential to remove the vertices. Using ordered sets can
    // help modify inner values
    while (!pq.empty()) {
        auto [u, v, w] = pq.top();
        pq.pop();
        parent[v] = u;
        // if this vertex is not visited, then we can safely visit the 
        // neighboring vertices and evaluate the nearest vertices.
        if (!visited[v]) {
            visited[v] = true;
            // Destination vertex is found, find the path and 
            // the return it with the minimum cost.
            if (v == dest_vertex) {
                int traverse = dest_vertex;
                while (traverse != -1) {
                    min_path.push_back(traverse);
                    traverse = parent[traverse];
                }
                return {w, min_path};
            } else {
                // A graph can be constructed based on various factors.
                // It can either be relation between vertices/or an explicit
                // graph input if we want to find the minimum value.
                for (auto &[next_v, next_w]: adj_graph[v]) {
                    if (!visited[next_v]) {
                        pq.push(make_tuple(v, next_v, w+next_w));
                    }
                }
            }
        }
    }
    // There is no solution to the problem, return empty vector.
    return {-1, vector<int>(0)};
}
```
## Bellman Ford's Algorithm.