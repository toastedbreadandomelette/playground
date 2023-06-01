def mod_power(number, exponent: int, mod: int) -> int:
    """
    Evaluates number raise to exponent in O(logn), where logn is logarithm of number
    n to the base 2

    >>> mod_power(2, 20, 100)
    76

    >>> mod_power(33, 39, 1000000007)
    322956718

    >>> mod_power('abc', 38, 23)
    Traceback (most recent call last):
        ...
    TypeError: Number is of type <class 'str'>
    """
    if type(exponent) is not int:
        raise TypeError('Number is of type %s' % type(number))
    if type(number) is not int and type(number) is not float:
        raise TypeError('Number is of type %s' % type(number))
    result = 1
    while exponent > 0:
        if exponent & 1:
            result *= number
            result %= mod
        number *= number
        number %= mod
        exponent >>= 1
    return result

def is_palindrome(string: str) -> bool:
    """
    Check if string is palidrome or not
    >>> is_palindrome('aba')
    True
    >>> is_palindrome('racecar')
    True
    >>> is_palindrome('reer')
    True
    >>> is_palindrome('abab')
    False
    """
    return True if all([ string[i] == string[len(string) - 1 - i] for i in range(len(string) // 2) ]) else False

def rabin_karp_hashing(string: str) -> int:
    """
    Generates a hash that identifies a string
    >>> rabin_karp_hashing('hello world')
    152629105316270
    """
    mod = 344555666677777 # a prime number
    prime = 1
    charset_prime = 131   # a prime number just greater than charset (considering charset is first 128 letters)
    hash = 0
    for char in string:
        hash += ((ord(char)+1) * prime)
        hash %= mod
        prime *= charset_prime
        prime %= mod
    return hash

def rabin_karp_substring_check(string: str, substring: str) -> int:
    """
    Check whether substring exists or not.

    Returns starting index if substring is found, else `-1`
    The substring checks all happens in `O(n + k)` complexity, where
    n: Size of the search string
    k: Size of the searched string
    
    >>> rabin_karp_substring_check('abcacbc', 'cbc')
    4
    >>> rabin_karp_substring_check('abcacbc', 'cbcde')
    -1
    >>> rabin_karp_substring_check('abcdddfcbcdedecbacbc', 'cbcded')
    7
    >>> rabin_karp_substring_check('abcdddfcbcdedecbacbc', 'abcddd')
    0
    >>> rabin_karp_substring_check('The quick brown fox jumps over the lazy dog.', 'over the lazy dog.') == 26
    True
    """
    if len(substring) > len(string):
        return -1
    charset_prime = 131
    mod = 344555666677777   # a prime number
    charset_prime_inverse = mod_power(charset_prime, mod - 2, mod)
    substring_hash = rabin_karp_hashing(substring)
    hash_check, prime = 0, 1

    for left in range(len(substring)):
        char = string[left]
        hash_check += ((ord(char)+1) * prime) + mod
        hash_check %= mod
        prime *= charset_prime
        prime %= mod

    if hash_check == substring_hash:
        return 0
    else:
        left = 0
        for right in range(len(substring), len(string)):
            hash_check += ((ord(string[right])+1) * prime) + mod
            hash_check %= mod
            hash_check -= (ord(string[left])+1)
            hash_check += mod
            hash_check %= mod
            hash_check *= charset_prime_inverse            # Dividing with prime value (or multiplying inverse)
            hash_check %= mod
            left += 1
            if hash_check == substring_hash:
                return left
        return -1

def rabin_karp_all_substring_check(string: str, substring: str) -> int:
    """
    Returns all the starting indexes having matching substring
    else an empty array
    The substring checks all happens in O(n + k) complexity, where
    ```
    n: Size of the search string
    k: Size of the searched string
    ```

    >>> rabin_karp_all_substring_check('abcacbcbc', 'cbc')
    [4, 6]
    >>> rabin_karp_all_substring_check('abcacbc', 'cbcde')
    []
    >>> rabin_karp_all_substring_check('abcdddfcbcdedecbacbc', 'cbcded')
    [7]
    >>> rabin_karp_all_substring_check('abcdddfcbcdedecbacbc', 'abcddd')
    [0]
    """
    if len(substring) > len(string):
        return -1
    charset_prime = 131
    mod = 344555666677777   # a prime number
    charset_prime_inverse = mod_power(charset_prime, mod - 2, mod)
    substring_hash = rabin_karp_hashing(substring)
    hash_check, prime = 0, 1

    for left in range(len(substring)):
        char = string[left]
        hash_check += ((ord(char)+1) * prime) + mod
        hash_check %= mod
        prime *= charset_prime
        prime %= mod

    matching_indexes = []
    if hash_check == substring_hash:
        matching_indexes.append(0)

    left = 0
    for right in range(len(substring), len(string)):
        hash_check += ((ord(string[right])+1) * prime) + mod
        hash_check %= mod
        hash_check -= (ord(string[left])+1)
        hash_check += mod
        hash_check %= mod
        hash_check *= charset_prime_inverse             # Dividing with prime value (or multiplying inverse)
        hash_check %= mod
        left += 1
        if hash_check == substring_hash:
            matching_indexes.append(left)
    return matching_indexes

def prefix_function(T: str) -> list:
    """
    Returns the list of numbers that denotes computed
    prefix values of a substring.
    >>> prefix_function('abcabcd')
    [0, 0, 0, 1, 2, 3, 0]
    >>> prefix_function('aabaaababaab')
    [0, 1, 0, 1, 2, 2, 3, 4, 0, 1, 2, 3]
    """
    prefix_values = [0]*len(T)
    for i in range(1, len(T)):
        # Starting prefix_value as i - 1
        j = prefix_values[i - 1]
        # Search whether there is a prefix that contains
        # T[i] character, if not, reduce the prefix.
        # and search within the prefix
        while j > 0 and T[i] != T[j]:
            j = prefix_values[j - 1]
        
        # If found, increment it.
        if T[i] == T[j]: j += 1
        # Assign the value
        prefix_values[i] = j

    return prefix_values

def kmp_pattern_search(S: str, T: str) -> list:
    """
    Search string, returns all indexes
    >>> kmp_pattern_search('abcacbcbc', 'cbc')
    [4, 6]
    >>> kmp_pattern_search('abcaacbcbc', 'a')
    [0, 3, 4]
    >>> kmp_pattern_search('abcacbc', 'cbcde')
    []
    >>> kmp_pattern_search('abcdddfcbcdedecbacbc', 'cbcded')
    [7]
    >>> kmp_pattern_search('abcdddfcbcdedecbacbc', 'abcddd')
    [0]
    """
    prefix_values = prefix_function(T)
    i, j = 0, 0
    substrings = []
    while i < len(S):
        while i < len(S) and S[i] != T[j]:
            i += 1
        if i < len(S):
            start = i
            while i < len(S) and j < len(T) and S[i] == T[j]:
                i += 1
                j += 1

            if j == len(T):
                substrings.append(i - len(T))
                
            i = start + prefix_values[j-1] + 1
            j = prefix_values[j-1]

    return substrings

if __name__ == "__main__":
    from doctest import testmod
    testmod()
