
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

def n_queens(n: int) -> list:
    """
    Solves n-queen problem
    >>> n_queens(4)
    [[0, 0, 1, 0], [1, 0, 0, 0], [0, 0, 0, 1], [0, 1, 0, 0]]
    >>> n_queens(8)
    [[1, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0, 1, 0], [0, 0, 0, 0, 1, 0, 0, 0], [0, 0, 0, 0, 0, 0, 0, 1], [0, 1, 0, 0, 0, 0, 0, 0], [0, 0, 0, 1, 0, 0, 0, 0], [0, 0, 0, 0, 0, 1, 0, 0], [0, 0, 1, 0, 0, 0, 0, 0]]
    """
    row_memo, col_memo, front_diag_memo, back_diag_memo = [False]*n, [False]*n, [False]*(2*n+1), [False]*(2*n+1)
    
    not_present_in_row = lambda row: not(row_memo[row])
    not_present_in_col = lambda col: not(col_memo[col])
    not_present_in_front_diag = lambda row, col: not(front_diag_memo[row + col])
    not_present_in_back_diag = lambda row, col, size: not(back_diag_memo[row + size - 1 - col])
    
    def solve(board: list, col: int) -> bool:
        if col == n:
            return True
        for row in range(n):
            if not_present_in_row(row) and not_present_in_col(col) and \
                not_present_in_front_diag(row, col) and not_present_in_back_diag(row, col, n):
                board[row][col] = 1
                row_memo[row] = col_memo[col] = front_diag_memo[row + col] = back_diag_memo[row + n - 1 - col] = True
                if solve(board, col + 1):
                    return True
                else:
                    board[row][col] = 0
                    row_memo[row] = col_memo[col] = front_diag_memo[row + col] = back_diag_memo[row + n - 1 - col] = False
        return False

    board = [[0]*n for row in range(n)]
    solve(board, 0)
    return board

def knight_tour(n: int) -> list:
    """
    Moves for a knight in a board: can a knight cover all the block
    in a `n x n` chess board
    
    Note: This function uses degrees of reachability to solve the knight tour problem.
    This is called as Warnsdorff's algorithm, to find a hamiltonian path.
    >>> knight_tour(6)
    [[2, 17, 30, 11, 8, 19], [29, 12, 1, 18, 31, 10], [16, 3, 36, 9, 20, 7], [13, 28, 15, 24, 35, 32], [4, 23, 26, 33, 6, 21], [27, 14, 5, 22, 25, 34]]
    >>> knight_tour(8)
    [[2, 63, 16, 25, 44, 59, 14, 27], [17, 24, 1, 64, 15, 26, 43, 58], [62, 3, 52, 45, 60, 57, 28, 13], [23, 18, 61, 48, 51, 46, 55, 42], [4, 49, 22, 53, 56, 41, 12, 29], [19, 34, 37, 50, 47, 54, 9, 40], [36, 5, 32, 21, 38, 7, 30, 11], [33, 20, 35, 6, 31, 10, 39, 8]]
    """
    # Moves are 3 blocks: 1 or 2 block(s) either along x or y axis 
    # and remaining block perpendicular to the previous move: Possible moves are 8
    moves = ((1, 2), (1, -2), (-1, 2), (-1, -2),
             (2, 1), (2, -1), (-2, -1), (-2, 1))

    """
    # Check whether value is within the range (both inclusive)
    """
    within_range_inclusive = lambda value, min_value, max_value: value >= min_value and value <= max_value
    """
    Check whether position is not out of place
    """
    is_a_valid_move = lambda move, x_position, y_position: \
        within_range_inclusive(x_position + move[0], 0, n - 1) and \
        within_range_inclusive(y_position + move[1], 0, n - 1)
    """
    Check whether the next move we're performing is not visited before
    """
    is_not_visited = lambda board, move, rindex, cindex: board[rindex + move[0]][cindex + move[1]] == 0
    # matrix to store number of position in which knight can visit place mat(i, j)

    board = [[0] * n for i in range(n)]

    def degree_at_position(rindex: int, cindex: int) -> int:
        return len([True for move in moves if is_a_valid_move(move, rindex, cindex)])

    deg_board = [[degree_at_position(i, j)
                  for j in range(n)] for i in range(n)]

    def modify_deg_board_of_neighbor(rindex: int, cindex: int, modify: int):
        """
        Modify degree of reachability of neighbors, given the knight considers
        visiting `board[rindex][cindex] (modify=1)` or discards visiting for
        solution `(modify=-1)`.
        """
        for move in moves:
            if is_a_valid_move(move, rindex, cindex):
                deg_board[rindex + move[0]][cindex + move[1]] += modify

    def get_all_possible_moves(board, rindex: int, cindex: int) -> list:
        """
        Get all possible next moves, and sort them according to their lowest degree of the board
        """
        move_list = [(rindex + move[0], cindex + move[1]) for move in moves \
            if is_a_valid_move(move, rindex, cindex) and is_not_visited(board, move, rindex, cindex)]
        move_list.sort(key=lambda tup: deg_board[tup[0]][tup[1]])
        return move_list
    
    sq_count = n*n

    def solve(board: list, rindex: int, cindex: int, count: int) -> bool:
        """
        Function that solves knight tour

        - Base case: if count goes beyond `n*n`, it's clear all boxes
        are visited.
        - Get all valid moves given current condition of board, sorted according
        to degree of their position in ascending order.
            - Move to this position and modify neighboring degree of reachability
            by subtracting 1.
            - If moving ahead gives the solution to the problem, then
            return True
            - else unvisit this vertex, and modify neighboring degree by adding 1
        - None of this worked, return False
        """
        if count > sq_count:
            return True

        for move in get_all_possible_moves(board, rindex, cindex):
            x, y = move
            board[x][y] = count
            modify_deg_board_of_neighbor(x, y, -1)
            if solve(board, x, y, count + 1):
                return True
            else:
                board[x][y] = 0
                modify_deg_board_of_neighbor(x, y, 1)
        return False

    solve(board, 0, 0, 1)
    return board

def subset_problem(ls: list, target_value: int) -> list:
    """
    A subset problem is a np-complete problem, that evaluates if there is a 
    subset that evaluates their sum to `target_value`
    returns an empty list if there is no solution.

    >>> subset_problem([3, 34, 4, 12, 5, 2], 9)
    [3, 4, 2]
    >>> subset_problem([3, 34, 4, 12, 5, 2], 19)
    [12, 5, 2]
    """
    def subset_by_bit_masking(ls: list, target_value: int) -> list:
        """
        >>> subset_problem([3, 34, 4, 12, 5, 2], 9)
        [4, 5]
        >>> subset_problem([3, 34, 4, 12, 5, 2], 19)
        [3, 4, 12]
        """
        get_list_by_mask = lambda ls, mask: [item for index, item in enumerate(ls) if ((mask & (1<<(index))) != 0)]

        mask = (1 << (len(ls)))
        for x in range(mask):
            test_list = get_list_by_mask(ls, x)
            if sum(test_list) == target_value:
                return test_list
        return []

    def subset_meet_in_middle(ls: list, target_value: int) -> list:
        """
        >>> subset_meet_in_middle([3, 34, 4, 12, 5, 2], 9)
        [3, 4, 2]
        >>> subset_meet_in_middle([3, 34, 4, 12, 5, 2], 19)
        [12, 5, 2]
        """

        def binary_search(array: list, target: int) -> int:
            """
            Searches in a sorted array
            """
            low, high = 0, len(array) - 1
            while low < high:
                mid = low + (high - low) // 2
                if array[mid][0] == target:
                    return mid
                elif array[mid][0] > target:
                    high = mid - 1
                else:
                    low = mid + 1
            return low if array[low][0] == target_value else -1

        get_list_by_mask = lambda ls, mask: [item for index, item in enumerate(ls) if ((mask & (1<<(index))) != 0)]
        length = len(ls)
        
        first_list, second_list = ls[:length//2], ls[length//2:]
        l_first_half, l_second_half = len(first_list), len(second_list)

        first_half = [(sum(get_list_by_mask(first_list, mask)), mask) for mask in range(1 << (l_first_half))]
        second_half = [(sum(get_list_by_mask(second_list, mask)), mask) for mask in range(1 << (l_second_half))]
        # optimization: loop on half having smaller size of the two, so that computation is faster
        # e.g., for array of size 39: (split in 19 (mask: 2^19 = 524288) + 20 (mask: 2^20 = 1048576)), 
        # 524288 * 20 > 1048576 * 19 ( here log_2(1048576) = 20 and log_2(524288) = 19)
        second_half.sort()
        for x in first_half:
            first_sum, first_mask = x
            index = binary_search(second_half, target_value - first_sum)
            if index != -1:
                return get_list_by_mask(first_list, first_mask) + get_list_by_mask(second_list, second_half[index][1])
        return []

    return subset_meet_in_middle(ls, target_value)

if __name__ == "__main__":
    from doctest import testmod
    testmod()
