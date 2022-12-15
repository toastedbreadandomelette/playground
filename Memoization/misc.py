def knight_probability(n: int, k: int, row: int, column: int) -> float:
    """
    Given a n x n chessboard, a knight moves k steps.
    Find the probability that the knight stays on the board after 
    moving k times.
    >>> knight_probability(3, 2, 0, 0)
    0.0625
    >>> knight_probability(3, 0, 0, 0)
    1.0
    >>> knight_probability(8, 6, 4, 4)
    0.27028656005859375
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

if __name__ == '__main__':
    from doctest import testmod
    testmod()
