#include <vector>
#include <queue>
#include <utility>
#include <algorithm>
#include <iostream>
#include <functional>
#include <cassert>

using namespace std;

/**
 * @brief Construct a new pair<int, vector<int>>kruskal minimum spanning tree object
 * 
 * @param edges list of edges
 * @returns pair of integer (minimum weight) and list of edges, denoting
 * minimum spanning tree of a given graph.
 */
pair<int, vector<tuple<int, int, int>>> 
    kruskal_minimum_spanning_tree(vector<tuple<int, int, int>>&edges, 
                                  int total_vertices) {
    using edge = tuple<int, int, int>;
    // sort the list in ascending order of their weights.
    sort(begin(edges), end(edges), [](const edge&first, const edge&second) {
        return get<2>(first) < get<2>(second);
    });

    // Find parent of a vertex, and assign the chain to the first parent.
    function<int(vector<int>&, int)>find_parent = 
        [&find_parent](vector<int>&parent, int vertex) {
        if (parent[vertex] != vertex) {
            parent[vertex] = find_parent(parent, parent[vertex]);
        }
        return parent[vertex];
    };

    // Link the parent of first and second vertex. A union operation
    function<void(vector<int>&, int, int)>un = 
        [&find_parent](vector<int>&parent, int f_vertex, int s_vertex) {
        int parent_first_vertex = find_parent(parent, f_vertex);
        int parent_second_vertex = find_parent(parent, s_vertex);
        if (parent_first_vertex != parent_second_vertex) {
            parent[parent_first_vertex] = parent_second_vertex;
        }
    };

    int minimum_weight = 0;
    vector<edge>tree;
    vector<int>parent(total_vertices + 1, 0);
    for (int i = 1; i <= total_vertices; ++i) {
        parent[i] = i;
    }

    int i = 0;
    while(i < edges.size() && tree.size() < total_vertices - 1) {
        auto [u, v, w] = edges[i++];
        int pu = find_parent(parent, u), pv = find_parent(parent, v);
        if (pu != pv) {
            un(parent, pu, pv);
            tree.push_back(make_tuple(u, v, w));
            minimum_weight += w;
        }
    }

    return {minimum_weight, tree};
}

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

/**
 * 
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
            // is not an immediate vertex, so this check.
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
            if (sub_graph_bridges.size()) {
                result.insert(result.end(), sub_graph_bridges.begin(), sub_graph_bridges.end());
            }
        }
    }

    return result;
}

/**
 * @brief Query for Lowest Common Ancestor
 * 
 * @param tree Tree graph
 * @param root root of the tree denoting starting node of the traversal
 * @param queries List of queries: given two vertices u and v
 * @return vector<int> List of answer queries
 */
vector<int> queries_for_lca(vector<vector<int>>&tree, int root, const vector<pair<int, int>>&queries) {
    vector<int> answers;
    vector<bool> visited(tree.size(), false);
    vector<int> left(tree.size(), -1), height(tree.size(), 0);
    
    /**
     * @brief Generate traversal graph, and construct an inorder traversal of n-ary tree
     * except that the traversal after visiting child node (and their subsequent children), 
     * everytime, we insert the current node again.
     * 
     * @param vertex current vertex
     * @param prev_vertex previous vertex
     * @param depth depth at which the current node is
     * @param lca array denoting the construction of inorder traversal
     * @param visited marker for visibility of vertex
     * @returns bool returns true if there is no cycle
     */
    function<bool (int, int, int, vector<pair<int, int>>&, vector<bool>&)> traverse_graph = 
        [&traverse_graph, &tree, &left, &height](int vertex, int prev_vertex, int depth, vector<pair<int, int>>&lca, vector<bool>&visited) {
        if (left[vertex] == -1) {
            left[vertex] = lca.size();
        }
        lca.push_back({vertex, depth});
        height[vertex] = depth;

        for (auto &next_vertex: tree[vertex]) {
            if (prev_vertex != next_vertex) {
                if(traverse_graph(next_vertex, vertex, depth + 1, lca, visited)) {
                    lca.push_back({vertex, depth});
                } else {
                    return false;
                }
            }
        }
        return true;
    };

    vector<pair<int, int>>lca;
    if (traverse_graph(root, -1, 1, lca, visited)) {
        vector<int>seg_tree(lca.size() - 1, 0);
        // Extend Segment tree by adding lca array.
        seg_tree.insert(seg_tree.end(), lca.begin(), lca.end());

        for (int i = lca.size() - 2; i >= 0; --i) {
            int left = seg_tree[2*i+1], right = seg_tree[2*i+2];
            seg_tree[i] = height[left] > height[right] ? right : left;
        }
        for (auto &[u, v]: queries) {
            int iu = left[u], iv = left[v];
            iu += lca.size() - 1, iv += lca.size() - 1;
            if (iu > iv) {
                tie(iu, iv) = make_tuple(iv, iu);
            }

            int mn = height[seg_tree[iu]];
            int ans = seg_tree[iu];
            for (; iu <= iv;) {
                if (!(iu & 1)) {
                    if (mn > height[seg_tree[iu]]) {
                        ans = seg_tree[iu];
                        mn = height[seg_tree[iu]];
                    }
                    ++iu;
                }
                if ((iv & 1)) {
                    if (mn > height[seg_tree[iv]]) {
                        ans = seg_tree[iv];
                        mn = height[seg_tree[iv]];
                    }
                    --iv;
                }
                --iu;
                iu >>= 1;
                --iv;
                iv >>= 1;
            }

            answers.push_back(ans);
        }
    }

    return answers;
}

void testing () {
    vector<vector<int>>adj(9);

    /**
     *  7         3
     *  | \     /   \
     *  |  1 - 2     5
     *  | /     \   /
     *  8         4
     * Expected output: 1 - 2
     */

    for (auto &[u, v]: vector<pair<int, int>>({{1, 2}, {1, 7}, {1, 8}, {7, 8}, {2, 3}, {4, 2}, {5, 3}, {5, 4}})) {
        adj[u].push_back(v);
        adj[v].push_back(u);
    }

    auto answer = find_all_bridges(adj, 1);
    cout << answer.size() << endl;
    for (auto &[u, v]: answer) {
        cout << u << " - " << v << endl;
    }
    cout << endl;

    /**
     *  7         3
     *    \     /   \
     *     1 - 2     5
     *    /     \   /
     *  8         4
     * Expected output: 
     * 1 - 2
     * 1 - 7
     * 1 - 8
     */

    adj = vector<vector<int>>(9);
    for (auto &[u, v]: vector<pair<int, int>>({{1, 2}, {1, 7}, {1, 8}, {2, 3}, {4, 2}, {5, 3}, {5, 4}})) {
        adj[u].push_back(v);
        adj[v].push_back(u);
    }

    answer = find_all_bridges(adj, 1);
    cout << answer.size() << endl;
    for (auto &[u, v]: answer) {
        cout << u << " - " << v << endl;
    }

    /**
     *            1
     *        /   |  \
     *       /    |   \
     *      2     3    4
     *   / | | \  |  / | \
     *  5  6 7  8 9 10 11 12
     *                   /  \
     *                  13  14
     */

    vector<vector<int>>tree(15);
    vector<pair<int, int>>edges = {
        {1, 2}, {1, 3}, {1, 4}, {2, 5}, {2, 6},
        {2, 7}, {2, 8}, {3, 9}, {4, 10}, {4, 11},
        {4, 12}, {12, 13}, {12, 14}};

    for (auto &[u, v]: edges) {
        tree[u].push_back(v);
    }
    auto answer_lca = queries_for_lca(tree, 1, {{7, 8}, {9, 11}, {10, 13}, {11, 7}, {13, 9}, {14, 4}});
    
    for (auto &x: answer_lca) {
        cout << x << endl;
    }
}

int main () {
    testing();
    return 0;
}
