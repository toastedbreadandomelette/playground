# Problem Reduction
**This might be an important point for solving the problems here. All the others might be helpful for direct solution but this might gain a new perspective of finding some patterns.**

## [[memoization_2d_1#Longest Increasing Paths in a Grid https leetcode com problems longest-increasing-path-in-a-matrix|Longest Increasing Path in a grid]].

The given problem can also be converted into a DFS:
Let $(i,j)$ tuple be considered as a unique number (for convenience we'll consider transform function $t(i,j,n,m)=i\cdot n+j$), then their adjacent graph $p$ can be computed as:

$$p[t(i,j,n,m)]=\lbrace t(x,y,n,m): (x,y)\in dr, x\in[1,n], y\in[1,m],A_{ij} < A_{xy}\rbrace$$

This turns out from finding longest path to finding the depth of a directed graph with no cycles. (Cycles in such a graph would not make sense, as it create a paradox; where parent root has value less than a child and the last child linked to root is less than the root; and would be a never ending increasing value). Note that there $\exists\ v: |p[v]|=0$. 

With this, we can also count incoming edges $in[v]\ \forall\ v\in p$, and we select those vertex $v$ such that $in[v]=0$.

The max-depth for a vertex $v$ (say $d(p,in,v)$) is defined as:

$$
d(p,in,v)=\begin{cases}
1,&|p[v]|=0\\
1+\max\limits_{(\omega\ \in\ p[v])}(d(p,in,\omega)),&|p[v]| >0
\end{cases}
$$

The overall sub-graphs having max depth is the answer:

$$res=\max\limits_{v\ \in\ p,\ in[v]=0}(d(p, in,v))$$
```python
def longest_increasing_path(self, matrix: List[List[int]]) -> int:
	"""
	Solves the current problem by converting the matrix into an
	adjacent graph. The function max_dep can be sped up by
	converting into a iterative function.
	"""
	m, n = len(matrix), len(matrix[0])
	sz = m*n
	mt = matrix
	# Adjacency matrix and memo for storing the values.
	p, memo = [[] for x in range(sz)], [1]*sz
	vis = [False]*sz
	in_deg_0 = set(list(range(sz)))
	# Transform 2-D grid indexes into a linear ones.
	transform = lambda x, y, n: x * n + y
	
	def max_dep(vis, vertex):
		vis[vertex] = True
		ans = 1
		for x in p[vertex]:
			if not vis[x]:
				ans = max(ans, 1 + max_dep(vis, x))
			else:
				ans = max(ans, 1 + memo[x])
		memo[vertex] = ans 
		return ans
		
	# Create an adjacent matrix with given condition
	# if adjacent values in a matrix is greater, then add
	# it as an edge, also, the outgoing vertex will have an
	# incoming edges, remove those.	
	for x in range(m):
		for y in range(n):
			v = transform(x,y,n)
			if x > 0 and mt[x-1][y] > mt[x][y]:
				p[v].append(v-n)
				in_deg_0.discard(v-n)
			if x < m-1 and mt[x+1][y] > mt[x][y]:
				p[v].append(v+n)
				in_deg_0.discard(v+n)
			if y > 0 and mt[x][y-1] > mt[x][y]:
				p[v].append(v-1)
				in_deg_0.discard(v-1)
			if y < n-1 and mt[x][y+1] > mt[x][y]:
				p[v].append(v+1)
				in_deg_0.discard(v+1)
		
	max_d = 0
	# Only consider those vertex that have zero incoming edges.
	for x in list(in_deg_0):
		max_d = max(max_d, max_dep(vis, x))

	return max_d
```

This solution is a bit different than mentioned in [[memoization_2d_1#Longest Increasing Paths in a Grid https leetcode com problems longest-increasing-path-in-a-matrix|here]], but the construction of adjacency graph $p$ to convert it to an directed graph with no loops: is what makes it different from the previous method.