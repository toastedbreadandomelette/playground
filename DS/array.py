from math import sqrt, floor, ceil

def segregate(arr: list, condition):
    """
    Segregates array based on condition
    >>> segregate([65, 55, 34, 41, 52, 26, 33, 30, 40], lambda x: x % 2 == 0) # split even and odd numbers
    [40, 30, 34, 26, 52, 41, 33, 55, 65]
    >>> segregate([65, 55, 34, 41, 52, 26, 33, 30, 40], lambda x: x % 5 == 0) # split divisors of 5
    [65, 55, 40, 30, 52, 26, 33, 41, 34]
    """
    if condition is None:
        raise Exception('No proper condition defined')
    i, j = 0, len(arr) - 1
    while i < j:
        while i < j and condition(arr[i]): i += 1
        while i < j and not condition(arr[j]): j -= 1
        arr[i], arr[j] = arr[j], arr[i]
    return arr

def preprocess(array: list, q_function) -> list:
    """
    Preprocesses the array such that for size n, we get sqrt(n)
    pre-processed array
    >>> preprocess([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16], sum)
    [10, 26, 42, 58]
    >>> preprocess([15, 23, 31, 45, 5, 36, 97, 28, 49, 110, 11, 52, 213, 14, 1, 10, 171, 41, 91, 21, 32, \
        232, 233, 111, 34, 24, 227, 81], min)
    [5, 28, 1, 10, 32, 24]
    """
    n = len(array)
    sqrt_array_len = int(sqrt(n))
    loop = sqrt_array_len + (0 if (floor(sqrt(n)) == sqrt(n)) else 1)
    return [q_function(array[ i * sqrt_array_len : min((i + 1) * sqrt_array_len, n)]) for i in range(loop)]

def sqrt_decomposition_solve_queries(array: list, queries: list, q_function) -> list:
    """
    Solves the queries using sqrt_decomposition.
    >>> a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
    >>> q_function = sum
    >>> queries = [[0, 10], [1, 3], [2, 15], [0, 16]]
    >>> sqrt_decomposition_solve_queries(a, queries, q_function)
    [55, 5, 117, 136]
    >>> a = [15, 23, 31, 45, 5, 28, 97, 36, 49, 110, 11, 52, 213, 14, 1, 10, 171, 41, 91, 21, 32, \
        232, 233, 111, 34, 24, 227, 81]
    >>> q_function = min
    >>> queries = [[0, 10], [1, 3], [2, 15], [0, 16], [21, 26], [5, 10]]
    >>> sqrt_decomposition_solve_queries(a, queries, q_function)
    [5, 23, 1, 1, 24, 28]
    """
    sqrt_array = preprocess(array, q_function)
    query_answers = []

    length = len(sqrt_array)
    is_perfect_sq = len(array) == len(sqrt_array) * len(sqrt_array)
    actual_sqrt = length if is_perfect_sq else length - 1

    for left, right in queries:
        sleft, sright = left // actual_sqrt, right // actual_sqrt
        # in the same sqrt block
        if sleft == sright:    
            query_answers.append(q_function(array[left:right]))
        # in different sqrt block
        else:
            value = None
            # left is not a perfect multiple of actual_sqrt
            # cannot be captured by sqrt block, so have to access part of array.
            if sleft * actual_sqrt < left:
            # complexity O(sqrt(n))
                value = q_function(array[left:(sleft+1)*actual_sqrt])
                sleft += 1          # skip this block since we considered it.

            # right is not a perfect multiple of actual_sqrt
            # cannot be captured by sqrt block, so have to access part of array again
            # complexity O(sqrt(n))
            if sright * actual_sqrt < right:
                if value is not None:
                    value = q_function([value, q_function(array[sright*actual_sqrt:right])])
                else:
                    value = q_function(array[sright*actual_sqrt:right])
            
            # Capture remaining by sqrt_array
            if sleft < sright:
                if value is not None:
                    value = q_function([value, q_function(sqrt_array[sleft:sright])]) #
                else:
                    value = q_function(sqrt_array[sleft:sright])
            query_answers.append(value)
    return query_answers

def difference_array_queries(array: list, diff_array: list, queries: list, operation, inv_operation) -> list:
    """
    Apply difference array queries to a empty array, so that 
    while display the results are shown with the changes
    (queries would be a tuple of left, right and value to change)

    diff_array is initalized as a pair of zeroes.

    >>> array = [1] * (25)
    >>> array
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
    >>> diff_array = [(0, 0)] * (25)
    >>> queries = [(1, 5, 2), (2, 6, 5), (12, 16, 10), (23, 24, 3)]
    >>> operation = lambda add_to, add_by: add_to + add_by
    >>> inv_operation = lambda add_to, add_by: add_to - add_by
    >>> difference_array_queries(array, diff_array, queries, operation, inv_operation)
    [1, 3, 8, 8, 8, 8, 6, 1, 1, 1, 1, 1, 11, 11, 11, 11, 11, 1, 1, 1, 1, 1, 1, 4, 4]

    >>> array = [2] * 12 + [3] * 13
    >>> diff_array = [(1, 1)] * (25)
    >>> queries = [(1, 5, 2), (2, 8, 5), (3, 16, 10), (23, 24, 3)]
    >>> operation = lambda multiply_to, multiply_by: multiply_to * multiply_by
    >>> inv_operation = lambda divide_to, divide_by: divide_to // divide_by
    >>> difference_array_queries(array, diff_array, queries, operation, inv_operation)
    [2, 4, 20, 200, 200, 200, 100, 100, 100, 20, 20, 20, 30, 30, 30, 30, 30, 3, 3, 3, 3, 3, 3, 9, 9]

    >>> array = [2] * 12 + [3] * 13
    >>> array
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3]
    >>> diff_array = [(0, 0)] * (25)
    >>> queries = [(1, 5, 2), (2, 8, 5), (3, 16, 10), (23, 24, 3)]
    >>> operation = lambda xor1, xor2: xor1 ^ xor2
    >>> difference_array_queries(array, diff_array, queries, operation, operation)
    [2, 0, 5, 15, 15, 15, 13, 13, 13, 8, 8, 8, 9, 9, 9, 9, 9, 3, 3, 3, 3, 3, 3, 0, 0]
    """
    initial_values = diff_array[0]
    # Processing queries takes O(q) time
    for L, R, value in queries:
        diff_array[L] = (operation(diff_array[L][0], value), diff_array[L][1]) if diff_array[L][0] else (value, diff_array[L][1])
        diff_array[R] = (diff_array[R][0], operation(diff_array[R][1], value)) if diff_array[R][1] else (diff_array[R][0], value)

    result = []
    value = initial_values[0]
    # Processing array takes O(n) time
    for index, values in enumerate(diff_array):
        if values[0] != 0:
            value = operation(value, values[0])
        result.append(operation(array[index], value))
        if values[1] != 0:
            value = inv_operation(value, values[1])

    return result

def difference_array_and_queries(array: list, queries: list) -> list:
    """
    Difference array for bitwise AND operator
    >>> array = [25] * 12 + [31] * 13
    >>> queries = [(1, 5, 21), (2, 8, 15), (3, 16, 10), (23, 24, 23)]
    >>> difference_array_and_queries(array, queries)
    [25, 17, 1, 0, 0, 8, 8, 8, 8, 8, 8, 8, 10, 10, 10, 10, 31, 31, 31, 31, 31, 31, 31, 23, 31]
    """
    def set_bits(bit_set_list: list, value: int, sub: bool) -> None:
        """
        Increase the count of bits that occur in value integer

        These are stored in bit_set_list at a particular index.
        """
        arr = []
        while value > 0:
            arr.append(value & 1)
            value >>= 1
        lenarray = len(arr)
        # Add remaining bits to the array
        arr.extend([0]*(32 - lenarray))
        if not sub:
            for x in range(32):
                bit_set_list[x] += arr[x]
        else:
            for x in range(32):
                bit_set_list[x] -= arr[x]
    
    def construct_bits(total_integers: int, current_bits_set: list, bits_set: list) -> int:
        """
        Construct integer from current bits set. 

        Checks whether total_integers that are ANDed with 
        the array is the same as current_bits_set and sets those
        bits accordingly.
        """
        integer = 0
        for index in range(32):
            current_bits_set[index] += bits_set[index]
        for index in range(31, -1, -1):
            integer <<= 1
            if current_bits_set[index] == total_integers:
                integer ^= 1
        return integer
    # Set bits take O(logn) time
    # Construct bits takes O(logn) time

    # assuming the integer bit-size is 32
    bits_set = [[0]*32 for i in range(len(array))]
    
    integers_added = [0]*len(array)
    # This takes O(q.log(max(value))) time
    for L, R, value in queries:
        set_bits(bits_set[L], value, False)
        integers_added[L] += 1
        set_bits(bits_set[R], value, True)
        integers_added[R] -= 1

    # This takes (O(n.log(max(value)))) time
    result = []
    current_bits_set = [0]*32
    total_integers = 0
    for index, values in enumerate(integers_added):
        total_integers += values
        integer = construct_bits(total_integers, current_bits_set, bits_set[index])
        result.append(array[index] & integer)
    # return 0
    return result

if __name__ == '__main__':
    from doctest import testmod
    testmod()
