# String
Sequence of characters (either ASCII/Unicode UTF-8). These are stored similar to [[arrays_1d|arrays]] as consecutive $8$-bit or $16$-bit array.

# Operations
## Palindrome
A recursive definition of palindrome $pldr$ is:
$$
pldr(S, L, R) = \left\{\begin{array} {cl}
\textrm{true},&\quad L\geq R\\
\textrm{false},&\quad L<R\wedge S[L]\neq S[R]\\
pldr(S, L+1, R-1),&\quad L<R\wedge S[L]=S[R]\\
\end{array}
\right.
$$
More formal definition of palindrome is a string that reads the same forwards and backwards.
By this definition
$$
pldr\textrm{(S)} = \left\{\begin{array} { c l }
	\textrm{true}, & \quad S=\textrm{ reverse}(S)\\
\textrm{false}, & \quad S \neq\textrm{ reverse}(S)
\end{array}
\right.
$$
```python
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
```

## Reverse
- [ ] To do.

# String [[searching#Substring Searching|Matching/Searching]].

## Rabin Karp method.

Rabin karp method is a way to identify a string with a certain integer value.
Each of the values is constructed as:
$$
\text{hash}(s) = \left(\sum\limits_{i=0}^{\text{len(s)}}s[i].p^{i}\right)(\bmod m)
$$
where $p$ and $m$ is a prime number. 
For a character set $C$ used in a string, prime number $p$ is selected such that $p > |C|$ where $|C|$ is size of that set.

Below example uses $p=131$ (since character set are used from $0$ to $127$ but we increment them by $1$, counting from $1$  to $128$), and $m=344555666677777$.

[A Good source written in C++](https://cp-algorithms.com/string/rabin-karp.html).

```python
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
```

These values would work for most of the string sets almost all the time.
Generating strings: Dumb but keep in head: There is a minimum string length $n$ above which generating all string would definitely collide hashes with one another.

E.g., For string of length $7$, number of ways to make a string of length $7$ from first $128$ characters: $128^7 = 562949953421312 > 344555666677777$ unique hashes.
i.e., By pigeonhole's principle, there are **at least** $562949953421312\text{ (eggs)} - 344555666677777\text{ (nests/holes)}=218394286743535\text{ (extra eggs)}$ strings that will collide with existing strings (eggs), although there might be more string that could collide. But for unique strings $\leq 10^7$, the probability of strings having same hash values is pretty low.

Also, having $344555666677777$ strings of length $7$ amounts to $2193.6 \text{ TB}$ of memory space. Absurdly Huge Bruh.

## Knuth-Morris-Pratt (KMP) Method.
It's one of the well known method to find a pattern in a string, using prefix function. We'll suppose $S$ to be the string to search and $T$ as substring pattern
### Prefix function
Prefix function is an array of length $|T|$ (say $\pi$). The prefix value of $i^{th}$ position is the next position from which substring check needs to be done.
This is done by checking substring $T[0\ldots i]$:
- If say, a last part of this substring (suffix) matches the first part (prefix): (i.e.,  iff $T[0\ldots k]=T[(i-k+1)\ldots i]$), then the value of $\pi[i]$ would be $k+1$.
- If there are more than one for $T[0\ldots i]$, the ones with max $k$ value is picked.

This is essential, since, after a mismatch of a substring, we are sure suffix and prefix of length $k$ is same, so we can directly skip $k$ characters from original string and substring checks.

```python
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
```

### Applications for prefix functions:

1. String searching.
	1. As mentioned with the help of prefix function, the substring checks can be done in faster time.

```python
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
```

Time Complexity of Pattern Searching: $O(|T|+|S|)$.
**Proof**: 
- [ ] To do.
2. Counting prefixes in a string.
Counting prefixes $T[0\ldots i]$ that occur in string $T$, also can be done for other string $S$.
- [ ] To do.

## Find $n^{th}$ lexicographical string.