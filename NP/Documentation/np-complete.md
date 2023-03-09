# NP - Complete
A problem is called NP-complete if a problem is:
- [[np|NP problem]] (decision problem, and decisive)
- [[np-hard|NP-hard]] (reducible from problem $L$ to $H$)

# Sudoku Puzzle.
Given a $9\times9$ sudoku board with some empty spaces, find whether the solution exists for the puzzle.
For all empty spaces (and filled spaces), the value should not repeat in that row, column and $3\times3$ grid.

```python
def sudoku(board: list) -> list:
    """
    Solves the 9x9 sudoku puzzle for you.

    Will return puzzle as it is if it does not find any solution.

    Solution for this is verifiable in `O(n^2)` where `n` is size 
    of the board `(here 9x9 = 9^2 = 81)`.

    >>> sudoku([[5, 3, 0, 0, 7, 0, 0, 0, 0], [6, 0, 0, 1, 9, 5, 0, 0, 0], [0, 9, 8, 0, 0, 0, 0, 6, 0], [8, 0, 0, 0, 6, 0, 0, 0, 3], [4, 0, 0, 8, 0, 3, 0, 0, 1], [7, 0, 0, 0, 2, 0, 0, 0, 6], [0, 6, 0, 0, 0, 0, 2, 8, 0], [0, 0, 0, 4, 1, 9, 0, 0, 5], [0, 0, 0, 0, 8, 0, 0, 7, 9]])
    [[5, 3, 4, 6, 7, 8, 9, 1, 2], [6, 7, 2, 1, 9, 5, 3, 4, 8], [1, 9, 8, 3, 4, 2, 5, 6, 7], [8, 5, 9, 7, 6, 1, 4, 2, 3], [4, 2, 6, 8, 5, 3, 7, 9, 1], [7, 1, 3, 9, 2, 4, 8, 5, 6], [9, 6, 1, 5, 3, 7, 2, 8, 4], [2, 8, 7, 4, 1, 9, 6, 3, 5], [3, 4, 5, 2, 8, 6, 1, 7, 9]]
    """
    hash = [0 for i in range(9)]
    for rindex, row in enumerate(board):
        for cindex, element in enumerate(row):
            if element != 0:
                hash[rindex] |= (1 << element)
                hash[cindex] |= (1 << (element + 9))
                hash[(rindex // 3) * 3 + (cindex // 3)] |= (1 << (element + 18))

    not_present_in_row = lambda hash, rindex, value: (hash[rindex] & (1 << value)) == 0
    not_present_in_col = lambda hash, cindex, value: (hash[cindex] & (1 << (value + 9))) == 0
    not_present_in_3x3 = lambda hash, rindex, cindex, value: \
        (hash[(rindex // 3) * 3 + (cindex // 3)] & (1 << (value + 18))) == 0

    def solve(board: list, hash: list, rindex: int, cindex: int) -> bool:
        """
        Solving sudoku board. Returns `True` if one solution exists for the
        board, else returns `False`

        Follows recursion:
        - If all values are visited (rindex == 9), then all values are 
        successfully placed (This makes assumptions that the values are carefully
        selected and assigned)
        - Else we'll go one by one by selecting each value from each row:
            - If there are no elements in row, select next row and see if current
            setup can yield result
            - If there is a value already assigned, then move to next element
            - Otherwise we'll have to make trial for each values from 1 to 9:
                - If there isn't any value equal to itself in same row, column or 
                in 3x3 row, then
                    - Mark the existence of this value in the row, column and 3x3 grid
                    for future calculation, and assign the value to the board
                    - If this step yields correct solution, then return True, else
                    erase value from board as well as existence of value from row, col
                    and 3x3 grid
        - None of this steps worked, return False
        """
        if rindex >= 9:
            return True
        elif cindex >= 9:
            if solve(board, hash, rindex + 1, 0):
                return True
        elif board[rindex][cindex] > 0:
            if solve(board, hash, rindex, cindex + 1):
                return True
        else:
            for value in range(1, 10):
                if not_present_in_row(hash, rindex, value) and \
                   not_present_in_col(hash, cindex, value) and \
                   not_present_in_3x3(hash, rindex, cindex, value):
                    hash[rindex] |= (1 << value)
                    hash[cindex] |= (1 << (value + 9))
                    hash[(rindex // 3) * 3 + (cindex // 3)] |= (1 << (value + 18))
                    board[rindex][cindex] = value

                    if solve(board, hash, rindex, cindex + 1):
                        return True
                    else:
                        hash[rindex] ^= (1 << value)
                        hash[cindex] ^= (1 << (value + 9))
                        hash[(rindex // 3) * 3 + (cindex // 3)] ^= (1 << (value + 18))
                        board[rindex][cindex] = 0
        return False
    
    solve(board, hash, 0, 0)
    return board
```

```ad-note
The below solution brute forces by checking and placing each and every values, so the worst case complexity would be $O(9^{9^2})$.

We can consider in general $n^2\times n^2$ for a certain $n$, with the same above set of rules. (or any $n\times n$ if $\sqrt{n}\notin \mathbb{N}$, then we'll have to ignore grid uniqueness).
```
