# Memoization 2D
Here, solving certain problems are dependent on two variables in a recurrence relation. We use [[arrays_1d|2D-array]] or hashmap of hashmaps for such kind of problems.
# Coin Change.
Given a set of coins, find
1. [[memoization_1d_1#Minimum amount of Coins|Minimum amount of coins to add up to certain value]] (This takes 1D Space)
2. Number of arrangements you can do to add upto that value
3. Number of combinations you can do to add upto that value

## Coin Combinations.
The problem statement aims to find different ways to make the coin sum to target value $T$ using coins from set $W$.

The problem boils down to whether we achieve target value: $1$, if $T=0$ (i.e., we found one of the arrangements), else perform operations if $T>0$, else for $T<0$, current tracking is not the valid solution.

Let us define the function $C(S,W,T,i)$, where $i$ is the $i^{th}$ coin.

How do we select the coins? We can either 
- Take the current coin and add it to the sum $C(S+W_i,W,T,i)$
- Skip this coin and find the next one $C(S,W,T,i+1)$

The answer would be sum of these values. 

**Note that** the function is similar to the finding any one solution, only difference is being adding all the combinations that lead to the sum $S$.

$$
C(S,W,T,i)=\left\{
\begin{array}{cl}
1,&S=T\\
0,&S>T\ \vee\ i\geq|W|\\
C(S,W,T,i+1)+C(S+W_i,W,T,i),&\text{otherwise}
\end{array}
\right.
$$

We can see that the variable $S$ and $i$ are two variables that are varying with the states, we can use these variables for storing the memo.


# [[graphs_1|Graphs]]

## [Longest Increasing Paths in a Grid](https://leetcode.com/problems/longest-increasing-path-in-a-matrix)

Given a matrix, return the length of longest path which are in *strictly* increasing order.
for e.g., $n=m=3$, and matrix $A$:

$$
\begin{bmatrix}
9&9&4\\6&6&8\\2&1&1
\end{bmatrix}
$$

The longest path is 4: i.e., $A_{32}\rightarrow A_{31}\rightarrow A_{21}\rightarrow A_{11}\implies 1\rightarrow2\rightarrow6\rightarrow9$.

The allowed directions are from $A_{xy}$ to $(A_{(x-1)y},A_{(x+1)y},A_{x(y-1)},A_{x(y+1)})$ given $x\pm1,y\pm1\in[1,n]$. Let $dr=[(x+1,y),(x-1,y),(x,y+1),(x,y-1)]$ be the list of possible directions. Then $\forall\ (x,y): \ x\in[1,n]\ ,y\in[1,m]$, the longest path $l(A,x,y)$ is calculated as maximum value from any one of these indices:

$$
l(A,x,y)=\left\{
\begin{array}{cl}
1,&\forall (r,c)\in dr, A_{rc}<A_{xy}\\
\max\limits_{(r,c)\ \in\ dr\ \wedge\ (r,c)\in[1,n]}(1+l(A,r,c)),&A_{rc}>A_{xy}
\end{array}
\right.
$$

Since there are two variables $x,y$ the function depends on, we'll need to store in a $2$-Dimensional memo as $memo_{xy}$.

Below code shows an iterative approach though. Stores in $2$ dimensional array.

```python
def longest_increasing_path(matrix: list) -> int:
	"""
	Return the longest path that is strictly in increasing order.
	"""
	m, n = len(matrix), len(matrix[0])
	mt = matrix
	visited = [[False]*n for x in range(m)]
	values = [[0]*n for x in range(m)]
	max_length = 0
	
	ir = lambda x, r: x >= 0 and x < r
	
	for x in range(m):
		for y in range(n):
			if visited[x][y] == False:
				stack = [(x, y)]
				while stack:
					r, c = stack.pop()
					dr = ((r+1, c), (r-1, c), (r, c+1), (r, c-1))
					if visited[r][c] == True:
						values[r][c] = 1
						for new_r, new_c in dr:
							if ir(new_r, m) and ir(new_c, n) and mt[r][c] < mt[new_r][new_c]:
								values[r][c] = max(values[r][c], 1+values[new_r][new_c])
								
						max_length = max(max_length, values[r][c])
						
					else:
						visited[r][c] = True
						stack.append((r, c))
					
						for new_r, new_c in dr:
							if ir(new_r, m) and ir(new_c, n) and mt[r][c] < mt[new_r][new_c] and visited[new_r][new_c] == False:
								stack.append((new_r, new_c))

	return max_length
```

A $1$-dimensional space can also be done (the time/space complexity will be the same) as follows, considering transforming $(i,j)$ to a $1$ dimensional indexes $(i\cdot n+j)$. Immediate value in the same column can be evaluated as $(i\pm n)$, and immediate values in a row can be evaluated as $(i\pm1)$.

```python
def longest_increasing_path(matrix: list) -> int:
	"""
	One dimensional approach, the matrix is flattened and same
	operations are performed.
	"""
	m, n = len(matrix)+2, len(matrix[0])+2
	sz = m*n
	mt = tuple(([0]*n) + sum([[0] + row + [0] for row in matrix], []) + ([0]*n))
	visited, values = [False]*(sz), [1]*sz
	# Creating boundaries so that these vertices should
	# not be computed.
	for x in range(0, sz, n):
		visited[x] = visited[x+n-1] = True
	for x in range(0, n):
		visited[x] = visited[sz-x-1] = True
		
	max_length = 0
	stack = []
	for p in range(1, sz):
		if not visited[p]:
			stack.append(p)
			while stack:
				v = stack.pop()
				# Immediate values in column (v (+-) n) and rows (v (+-) 1)
				dr = (v+n, v-n, v+1, v-1)
				# This condition is checked to merge all the results
				# of the neighboring values.
				if visited[v]:
					for new_v in dr:
						if mt[v] < mt[new_v]:
							values[v] = max(values[v], 1 + values[new_v])
					# Max length is computed here.
					max_length = max(max_length, values[v])

				else:
					visited[v] = True
					# We push the current vertex at the beginning, because we
					# need to merge all the results of their neighboring values.
					stack.append(v)
					stack.extend([new_v for new_v in dr if not visited[new_v] and mt[v] < mt[new_v]])

	return max_length
```

The complexity does not differ in both cases: $O(m\times n)$.

We also have another solution, but we converts the problem into a [[problem_reduction_1#memoization_2d_1 Longest Increasing Paths in a Grid https leetcode com problems longest-increasing-path-in-a-matrix Longest Increasing Path in a grid|graph]].

