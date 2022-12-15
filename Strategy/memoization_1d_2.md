# $1$-Dimesional Memoization
First part is [[memoization_1d_1|here]].
# Graphs
## [Total unique BST's](https://leetcode.com/problems/unique-binary-search-trees/) of $n$ nodes

Given $n$ nodes, determine the unique BSTs.
For e.g., $n=3$, there are 5 solutions.

To evaluate the value for $n$ nodes, we'll need to determine leaving root node, number of left sub-tree node combination as well as right tree node combination.
E.g., for $n=3$, we leave the root and we're left with $n-1=2$ nodes. The possible node combination $c$ for left $L$ and $R$ configuration would be $(v,n-1-v),\ \forall\ v=1\rightarrow(n-1)$ i.e., ($(0,2), (1,1)$, and $(2,0)$).

These left and right subtrees are computed recursively. And for each of the $c(v)$ left unique arrangement, there are $c(n-1-v)$ subtrees.

$$
\therefore c(n)=\left\{
\begin{array}{cl}
1,&n\leq1,n\in\mathbb{N}\\
\sum\limits_{v=0}^{n-1}\left(c(v)\cdot c(n-1-v)\right),&n\geq2,n\in\mathbb{N}
\end{array}
\right.
$$
This is $1$-Dimensional so it requires only $1$ dimensional memoization.

```python
def tree(n: int, memo: dict):
	"""
	Returns the solution of total BST solutions.
	>>> tree(1)
	1
	>>> tree(3)
	5
	>>> tree(5)
	42
	>>> tree(19)
	1767263190
	"""
	if n <= 1:
		return 1
	else:
		if n not in memo:
			memo[n] = sum([(tree((n-1)-x, memo) * tree(x, memo)) for x in range(n)])
		return memo[n]
```

Note that this is a [Catalan series](https://en.wikipedia.org/wiki/Catalan_number#:~:text=In%20combinatorial%20mathematics%2C%20the%20Catalan,Catalan%20(1814%E2%80%931894).).

# Maximum depth of a [[trees|tree]].
There is a similar solution to a [[problem_reduction_1#memoization_2d_1 Longest Increasing Paths in a Grid https leetcode com problems longest-increasing-path-in-a-matrix Longest Increasing Path in a grid|a modified problem]].