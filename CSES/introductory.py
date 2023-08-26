def weird_algorithm(n: int) -> int:
    """
    Printing all values generated till n reaches 1
    >>> weird_algorithm(3)
    3 10 5 16 8 4 2 1
    """
    while n != 1:
        print(n, end=' ')
        n = 3*n + 1 if n & 1 == 1 else n >> 1
    print(1)

def missing_number(n:int, array: list) -> int:
    """
    Prints missing number: Note array contains from 1 to n but one is missing
    >>> missing_number(5, [2, 3, 1, 5])
    4
    >>> missing_number(10, [2, 3, 1, 5, 6, 9, 7, 4, 10])
    8
    """
    array_sum = sum(array)
    return (n*(n + 1)) // 2 - array_sum

def repetitions(dna: str) -> int:
    """
    Prints longest consecutive same characters
    >>> repetitions('ATTCGGGA')
    3
    >>> repetitions('ATTCGGGAAAABBBBBCDE')
    5
    >>> repetitions('ATTCGGGAAAABBBBBB')
    6
    """
    dna_len, max_len = len(dna), 0
    start, cchar = 0, dna[0]
    for i in range(1, dna_len):
        if cchar != dna[i]:
            max_len, start, cchar = max(max_len, i - start), i, dna[i]
    if start != dna_len:
        max_len = max(max_len, dna_len - start)
    return max_len

def increasing_array(n: int, array: list) -> int:
    """
    Returns the increments to make array increasing
    >>> increasing_array(5, [3, 2, 5, 1, 7])
    5
    >>> increasing_array(10, [1000000000, 1, 1, 1, 1, 1, 1, 1, 1, 1])
    8999999991
    >>> increasing_array(1, [321421321])
    0
    """
    count = 0
    for i in range(1, len(array)):
        if array[i] < array[i - 1]:
            count += (array[i - 1] - array[i])
            array[i] = array[i - 1]
    return count

def permutations(n: int):
    """
    >>> permutations(4)
    [3, 1, 4, 2]
    >>> permutations(5)
    [3, 1, 4, 2, 5]
    >>> permutations(7)
    [4, 1, 5, 2, 6, 3, 7]
    >>> permutations(10)
    [6, 1, 7, 2, 8, 3, 9, 4, 10, 5]
    """
    if n <= 3:
        return 'NO SOLUTION'
    else:
        return [((n + i + int((n & 1) != 1)) // 2) if i % 2 != 0 else (i + 1) // 2 for i in range(1, n + 1)]

def number_spiral(x: int, y: int) -> int:
    """
    A spiral number grid is constructed as 
    ```
     1   2   9  10  25
     4   3   8  11  24
     5   6   7  12  23
    16  15  14  13  22
    17  18  19  20  21
    ```
    Find the number at xth row and yth column
    >>> number_spiral(2, 3)
    8
    >>> number_spiral(1, 1)
    1
    >>> number_spiral(4, 2)
    15
    >>> number_spiral(6, 1)
    36
    >>> number_spiral(1, 6)
    26
    >>> number_spiral(170550340, 943050741)
    889344699930098742
    """
    max_r = max(x, y)
    if max_r & 1:
        return max_r**2 - (max_r - y) - (x - 1)
    else:
        return max_r**2 - (max_r - x) - (y - 1)

def two_knights(n: int) -> list:
    """
    Number of ways knights can be placed so that they never 
    attack each other.
    >>> two_knights(8)
    [0, 6, 28, 96, 252, 550, 1056, 1848]
    >>> two_knights(10)
    [0, 6, 28, 96, 252, 550, 1056, 1848, 3016, 4662]
    """
    def inner_square(n: int):
        """
        ```
        Inner square of size (n - 4) has degree 8: i.e., a knight
        can be attached via 8 positions. subtract size to value from 
        total positions in the board. Such knights can be placed in
        (n - 4)**2 places
        ```
        """
        return ((n - 4)**2) * (n**2 - 9)
    
    def segment_adjacent_to_inner_square(n: int) -> int:
        """
        ```
        Segments adjacent to inner square has degree of 6: note that 
        there are only 4 strips of length (n - 4). Leave these 6 positions
        and place the other knight in other places
        ```
        """
        return ((n - 4)*4) * (n**2 - 7)

    def intersection_of_inner_segments(n: int) -> int:
        """
        ```
        Corners have degree 4, and there are only 4 in the chess board.
        ```
        """
        return (4) * (n**2 - 5)

    def outer_segment_of_size_n_minus_4(n: int) -> int:
        """
        These also have degree 4, there are 4 segments of size (n - 4)
        """
        return (n - 4)*(4) * (n**2 - 5)

    def blocks_adjacent_to_corners(n: int) -> int:
        """
        Deg: 3, 8 of them in whole board
        """
        return 8 * (n**2 - 4)

    def corners(n: int) -> int:
        """
        Deg: 2, only corners
        """
        return 4 * (n**2 - 3)

    if n == 1:
        return [0]
    return [0] + [(inner_square(i) + \
                   segment_adjacent_to_inner_square(i) + \
                   intersection_of_inner_segments(i) + \
                   outer_segment_of_size_n_minus_4(i) + \
                   blocks_adjacent_to_corners(i) + \
                   corners(i)) // 2 for i in range(2, n + 1)]

def two_sets(n: int) -> int:
    """
    Notion: The sets can be divided if sum `n * (n + 1) // 2`
    can be divided in half, otherwise nope...
    
    To divide it in two sets:
    - If n is divisible by 4:
        - `First set = [1, 2, 3 ... n // 4 - 1] + [3*n // 4 + 1, .... n]`
        - `Second set = [n/4 + 1, ..., 3*n/4 - 1]`
        
    - Else
        - From `n -> n // 2`, get number such that minimum value in first set
        is greater than `(n * (n + 1) // 4) - sum(first_set)` (the sum we want
        to achieve)

    >>> two_sets(7)
    ([7, 6, 1], [2, 3, 4, 5], 3, 4)
    >>> two_sets(8)
    ([1, 2, 7, 8], [3, 4, 5, 6], 4, 4)
    >>> two_sets(11)
    ([11, 10, 9, 3], [1, 2, 4, 5, 6, 7, 8], 4, 7)
    >>> two_sets(15)
    ([15, 14, 13, 12, 6], [1, 2, 3, 4, 5, 7, 8, 9, 10, 11], 5, 10)
    >>> two_sets(19)
    ([19, 18, 17, 16, 15, 10], [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 13, 14], 6, 13)
    """
    first, second, len_first, len_second = [], [], 0, 0
    if n % 4 == 0:
        first = [i for i in range(1, (n // 4) + 1)] + \
            [i for i in range((3*n)//4 + 1, n + 1)]
        second = [i for i in range(n // 4 + 1, (3*n) // 4 + 1)]
        len_first = len_second = n // 2
    elif (n + 1) % 4 == 0:
        max_val, divide = n, (n * (n + 1)) // 4
        while max_val < divide:
            first.append(max_val)
            divide -= max_val
            max_val -= 1
        first.append(divide)
        len_first = n - max_val + 1
        second = [i for i in range(1, max_val + 1) if i != divide]
        len_second = max_val - 1
    return first, second, len_first, len_second

def bit_string(p: int) -> int:
    """
    Exponentiation, mod 1e9 + 7
    >>> bit_string(10)
    1024
    >>> bit_string(20)
    1048576
    """
    n, result, mod = 2, 1, int(1e9 + 7)
    while p > 0:
        if p & 1:
            result = (result * n) % mod
        n, p = (n * n) % mod, p >> 1
    return result
        
def trailing_zeros(n: int) -> int:
    """
    Count number of trailing zeros in n!
    >>> trailing_zeros(10)
    2
    >>> trailing_zeros(100)
    24
    >>> trailing_zeros(125)
    31
    """
    _5, sum, add = 5, 0, n // 5
    while add > 0:
        sum, add, _5 = sum + add, n // (5*_5), _5*5
    return sum

def coin_piles(a: int, b: int) -> str:
    """
    Check whether pile of coins can be emptied by:
    - Pick one from either pile and two from the other pile.

    >>> coin_piles(4, 8)
    'YES'
    >>> coin_piles(5, 7)
    'YES'
    >>> coin_piles(4, 7)
    'NO'
    """
    return 'YES' if (a + b) % 3 == 0 and min(a, b) >= (a + b) // 3 else 'NO'

def palindrome_reorder(s: str) -> str:
    """
    Reorder to make a string palindrome
    """
    strlen = len(s)
    mpcount = { char: s.count(char) for char in 'ABCDEFGHIJKLMNOPQRSTUVWXYZ' }
    allow, odd_char, odd_char_count = True, 0, 0
    for char, count in mpcount.items():
        if count % 2 != 0:
            if (strlen & 1 and allow == False) or strlen % 2 == 0:
                return 'NO SOLUTION'
            else:
                allow = False
                odd_char, odd_char_count = char, count
    str = ''
    for char, count in mpcount.items():
        if char != odd_char:
            str += char*(count // 2)
    if strlen & 1:
        str += odd_char*(odd_char_count // 2)
        return '%s%s%s' % (str, odd_char, str[::-1])
    return '%s%s' % (str, str[::-1])

def gray_code(n: int) -> list:
    """
    Generate gray code.
    To generate the code, see the trailing zeros.
    Also, for array 
    ```
    ['0', '1'],
    ```
    - Create copy and reverse the new array
    - Append zero to first array
    - append one to new copied array
    Repeat this process till you get desired output of length (1 << n)

    >>> gray_code(2)
    ['00', '01', '11', '10']
    >>> gray_code(3)
    ['000', '001', '011', '010', '110', '111', '101', '100']
    """
    ls = ['0'.zfill(n)]
    bstrip = lambda x: len(x) - len(x.rstrip('0'))
    for x in range(1, (1 << n)):
        ls.append(bin(int(ls[-1], 2) ^ (1 << bstrip(bin(x))))[2:].zfill(n))
    return ls

def creating_strings(string: str) -> list:
    """
    Generate permutations of string s.
    >>> creating_strings('aabac')
    ['aaabc', 'aaacb', 'aabac', 'aabca', 'aacab', 'aacba', 'abaac', 'abaca', 'abcaa', 'acaab', 'acaba', 'acbaa', 'baaac', 'baaca', 'bacaa', 'bcaaa', 'caaab', 'caaba', 'cabaa', 'cbaaa']
    >>> creating_strings('abcd')
    ['abcd', 'abdc', 'acbd', 'acdb', 'adbc', 'adcb', 'bacd', 'badc', 'bcad', 'bcda', 'bdac', 'bdca', 'cabd', 'cadb', 'cbad', 'cbda', 'cdab', 'cdba', 'dabc', 'dacb', 'dbac', 'dbca', 'dcab', 'dcba']
    """
    def next_permutation(char_array: list, strlen: int) -> str:
        """
        Generates next lexicographically greater array permutation, given the current array.

        Process (e.g., for string `'abcba'`):
        - From the tail of array, get an index where the suffix is in decreasing order 

        ```
        for string 'abcba', last three values are in decreasing order: index is 2
        and the string is split in two parts: 
        => str[:index]('ab')
        => str[index:]('cba'))
        ```
        - Get the next character from second half string just greater than the last character of the 
        first half:

        ```
        first_half = 'ab' 
        => first_half[-1] = 'b' # last character
        => next character from second half just greater than 'b' = 'c')
        ```
        - Swap these two characters:

        ```
        => first_half = 'ac'
        => second_half = 'bba'
        ```
        - Sort the second half

        ```
        => second_half = 'abb'
        ```
        """
        shuffle_point, min_char, min_char_position, iter2 = strlen - \
            1, '{', strlen - 1, strlen - 1
        while shuffle_point > 0 and char_array[shuffle_point] <= char_array[shuffle_point - 1]:
            shuffle_point -= 1
        if shuffle_point == 0:
            return None
        while iter2 >= shuffle_point - 1:
            if char_array[iter2] > char_array[shuffle_point - 1] and min_char > char_array[iter2]:
                min_char, min_char_position = char_array[iter2], iter2
            iter2 -= 1
        char_array[shuffle_point -
                   1], char_array[min_char_position] = char_array[min_char_position], char_array[shuffle_point - 1]
        return char_array[:shuffle_point] + sorted(list(char_array[shuffle_point:]))

    char_array, strlen = sorted(list(string)), len(string)
    permutation_list = [''.join(char_array)]
    while char_array is not None:
        char_array = next_permutation(char_array, strlen)
        if char_array is None:
            break
        permutation_list.append(''.join(char_array))
    return permutation_list

def apple_division(n: int, apple_weight: list) -> list:
    """
    Divide weights of apple into two sets such that their weight 
    differences are minimum.
    ```
    let n = sum of all apple weights
    let p = sum of first set
    => q = n - p
    Difference between two sets = abs(p - (n - p)) = abs(2*p - n)
    ```
    >>> apple_division(5, [3, 2, 7, 4, 1])
    1
    >>> apple_division(6, [3, 2, 7, 4, 1, 6])
    1
    >>> apple_division(7, [3, 2, 7, 4, 1, 6, 7])
    0
    >>> apple_division(12, [3, 2, 7, 4, 1, 6, 70, 33, 56, 10, 66, 43])
    1
    """
    def weight_picker(apple_weight: list, mask: int, n: int):
        """
        Picks apples with certain weight based on mask
        """
        return sum([apple_weight[index] for index in range(n) if (mask & (1 << index)) > 0])

    def optimal_weight_binary_search(bit_mask_list: list, weight: int, weight_total_sum: int, n: int) -> int:
        """
        Modified binary search for searching minimum weight difference.
        """
        low, high = 0, n
        while low + 1 < high:
            mid = (low + high) // 2
            weight_diff = weight_total_sum - 2 * (weight + bit_mask_list[mid])
            if weight_diff > 0:
                low = mid
            elif weight_diff < 0:
                high = mid
            else:
                return mid
        if low + 1 < n:
            if abs(weight_total_sum - 2 * (weight + bit_mask_list[low])) > \
                    abs(weight_total_sum - 2 * (weight + bit_mask_list[low + 1])):
                return low + 1
        return low

    def apple_division_by_bit_masking(n: int, apple_weight: list) -> int:
        """
        Calculates answer by bitmasking all the values
        Much slower method since it enumerates all the combination 
        `(2^(n-1) iterations for n numbers)`
        Complexity: `O(n * 2^(n - 1))`
        """
        weight_total_sum = sum(apple_weight)
        return min([abs(weight_total_sum - 2*weight_picker(apple_weight, bin(mask)[2:].zfill(n), n)) for mask in range(0, (1 << (n - 1)))])

    def apple_division_meet_in_the_middle(n: int, apple_weight: list) -> list:
        """
        Calculating bitmask of half the values in first and second half in
        second list. Much faster method:

        Complexity:
        - Bitmask generation for half values: `O(2^(n // 2))`
        - iterate through loop: `O(n)`
        - Binary Search: `O(log(2^(n//2))) => O(n//2. log(2)) = O(n)`
        - => Overall complexity: `O(2^(n // 2))`

        """
        weight_total_sum = sum(apple_weight)
        first_half, second_half = apple_weight[:n // 2], apple_weight[n // 2:]
        flen, slen = len(first_half), len(second_half)
        bm_first = [weight_picker(first_half, mask, flen)
                    for mask in range(1 << (flen))]
        bm_second = [weight_picker(second_half, mask, slen)
                     for mask in range(1 << (slen))]
        bm_second.sort()

        min_weight = weight_total_sum
        for x in range(1 << flen):
            index = optimal_weight_binary_search(
                bm_second, bm_first[x], weight_total_sum, (1 << slen))
            if index < (1 << slen):
                min_weight = min(min_weight, abs(
                    weight_total_sum - 2*(bm_first[x] + bm_second[index])))
        return min_weight
    return apple_division_meet_in_the_middle(n, apple_weight)

def chessboard_and_queens(grid: list) -> int:
    """
    Generates all 8x8 queen chessboard solutions, given reserved spaces
    where queens cannot be placed (but does not block other queen from 
    attacking)
    >>> chessboard_and_queens(['........', '........', '..*.....', '........', '........', '.....**.', '...*....', '........'])
    65
    """
    row, col, front_diagonal, back_diagonal = [
        False for i in range(8)], [False for i in range(8)], [False for i in range(17)], [False for i in range(17)]
    
    present_in_row, present_in_col = lambda x: row[x], lambda y: col[y]
    present_in_front_diagonal = lambda x, y: front_diagonal[x + y]
    present_in_back_diagonal = lambda x, y: back_diagonal[x + 7 - y]

    ref_integer = [0]
    def count(grid: list, c: int, ref_count: list) -> int:
        if c == 8:
            ref_count[0] += 1
            return
        for r in range(8):
            if grid[r][c] == '.':
                if not(present_in_row(r)) and not(present_in_col(c)) and \
                    not(present_in_front_diagonal(r, c)) and not(present_in_back_diagonal(r, c)):
                    row[r] = col[c] = front_diagonal[r + c] = back_diagonal[r + 7 - c] = True
                    count(grid, c+1, ref_count)
                    row[r] = col[c] = front_diagonal[r + c] = back_diagonal[r + 7 - c] = False

    count(grid, 0, ref_integer)
    return ref_integer[0]

def digit_queries(n: int) -> int:
    """
    A string is generated by writing natural numbers one after another
    01234567891011121314151617181920...
    This function finds the nth digit in this string.
    >>> digit_queries(11)
    '0'
    >>> digit_queries(190)
    '1'
    >>> digit_queries(189)
    '9'
    """
    dig_length, position, multiplier = 1, 1, 1
    while position + dig_length * (multiplier * 10 - multiplier) < n:
        position += dig_length * (multiplier * 10 - multiplier)
        dig_length += 1
        multiplier *= 10
    number_skip, remainder = (
        (n - position) // dig_length), (n - position) % dig_length
    return str(number_skip + multiplier)[remainder]

def grid_paths(string: str) -> int:
    """
    Fast enough for pypy (not python3): Grid path
    >>> grid_paths('??????R??????U??????????????????????????LD????D?')
    201
    """
    idx = ['LRUD?'.index(c) for c in string]
    i = 0
    while i < 48 and idx[i] == 4:
        i += 1
    if i == 48:
        return 88418
    elif 2 * i >= 48:
        idx.reverse()
        for m in range(48):
            if idx[m] <= 1:
                idx[m] = 1 - idx[m]

    idx = tuple(idx)

    g = [0] * 81
    for x in range(0, 9):
        g[x] = g[72 + x] = g[x * 9] = g[x * 9 + 8] = 1
    
    moves, count, s, vertex, total_jumps = (-1, 1, -9, 9), 0, [10], 0, 0
    vertex, total_jumps = 0, 0
    while s:
        vertex = s.pop()
        if g[vertex]:
            g[vertex] = 0
            total_jumps -= 1
            continue
        if vertex == 64:
            if total_jumps == 48: count += 1
            continue
        if not(g[vertex + 9]) and not(g[vertex - 9]) and g[vertex + 1] and g[vertex - 1]:
            continue
        if not(g[vertex + 1]) and not(g[vertex - 1]) and g[vertex + 9] and g[vertex - 9]:
            continue
        if total_jumps == 48:
            continue
        g[vertex] = 1; s.append(vertex)
        if idx[total_jumps] == 4:
            if not(g[vertex + 1]):
                s.append(vertex + 1)
            if not(g[vertex - 9]):
                s.append(vertex - 9)
            if not(g[vertex + 9]):
                s.append(vertex + 9)
            if not(g[vertex - 1]):
                s.append(vertex - 1)
        else:
            if not(g[vertex + moves[idx[total_jumps]]]):
                s.append(vertex + moves[idx[total_jumps]])
        if s[-1] == vertex:
            g[vertex] = 0
            s.pop()
        else:
            total_jumps += 1
    return count

if __name__ == "__main__":
    from doctest import testmod
    testmod()

