# Memoization Miscellaneous
# Knight [[Probability]]
Given a knight in a $n\times n$ chessboard, return the probability that after $k$ moves, the knight stays in the chessboard.

In a chess, a knight changes it posiiton $b=1$ or $b=2$ blocks in one direction and remaining $3-b$ block in a direction perpendicular to the previous directions. (one move)
i.e., from $(i,j)$, there are $8$ possible moves: $\{(i\pm1,j\pm2),(i\pm2,j\pm1)\}=M$ (say). 
$N(T_1)$: The number of possible moves when $(k=1)=8$
$N(T_2)$: The number of possible moves when $(k=2)=8^2=64$
$\vdots$
$N(T_k)$: The number of possible moves is $=8^k$
$N(A_{ijk})$: The number of possible moves that are within chessboard when knight position is $(i,j)$. 
then $P(A_{ijk})=\dfrac{N(A_{ijk})}{N(T_k)}$.

Generalizing this. for $P(A_{ijk})=\dfrac{N(A_{ijk})}{8^k}$.
We need to evaluate $N(A_{ijk})$, which can be done via memoization.
# Algorithm:
- We perform the below operations $k$ times:
	- From $i,j$, we add the number of values since these are the ways that knight can end up.
		i.e., $\forall\ (i,j)\in[1,n],$ we perform the following operations
	$$
	\begin{matrix}
	\forall (x,y)\in M, (x,y)\in[1,n],memo_{ij}>0,\\
	memo\_next_{xy}:=memo\_next_{xy}+memo_{ij}
	\end{matrix}
	$$
	- The $memo\_next$ will be used in place of $memo$, and the $memo$ is discarded.

This will take space $O(n^2)$. The time complexity will be $O(n^2\cdot k)$.

Below code takes $k$ as a state too, which makes space complexity $O(n^2\cdot k)$.

```python
def knight_probability(n: int, k: int, row: int, column: int) -> float:
    """
    Calculates the probability of knight staying in the board.
    >>> knight_probability(3, 2, 0, 0)
    0.06250
    >>> knight_probability(3, 0, 0, 0)
    1
    >>> knight_probability(8, 6, 4, 4)
    0.27029
    """
    # memo to store the values
    dp = [[[0]*n for i in range(n)] for i in range(k+1)]
    dp[0][row][column] = 1
    # If index are in range
    ir = lambda x, y, n: x >= 0 and x < n and y >= 0 and y < n
    # Possible moves
    moves = ((1, 2), (2, 1), (-2, 1), (-1, 2), (1, -2), (2, -1), (-1, -2), (-2, -1))
    for i in range(k):
        for j in range(n):
            for l in range(n):
                if dp[i][j][l] > 0:
                    for dx, dy in moves:
                        x, y = j+dx, l+dy
                        if ir(x, y, n):
                            dp[i+1][x][y] += dp[i][j][l]

    sm = 0
    # Total moves are the total count of value in dp[k]
    for x in range(n):
        for y in range(n):
            sm += dp[k][x][y]
    # Return the probability
    return sm/(8**k)
```
