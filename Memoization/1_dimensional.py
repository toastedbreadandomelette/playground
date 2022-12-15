def longest_valid_parantheses(string: str) -> int:
    """
    Determine substring containing balanced parantheses
    >>> longest_valid_parantheses('))(())')
    4
    >>> longest_valid_parantheses(')()())')
    4
    >>> longest_valid_parantheses(')()()(()))')
    8
    >>> longest_valid_parantheses(')()())()()(')
    4
    """
    def longest_naive(string: str) -> int:
        """
        Naive search, takes O(n^2)
        """
        n, i, max_length = len(string), 0, 0
        # Till max_length, since there's no point in trying out 
        # next values
        while i < n - max_length:
            j, balance = i, 0
            while j < n and balance >= 0:
                if string[j] == '(': balance += 1
                else: balance -= 1
                if balance == 0:
                    max_length = max(max_length, j - i + 1)
                j += 1
            i += 1
        return max_length

    def longest_valid_parentheses_recursive(memo: dict, string: str, position: int) -> int:
        """
        Searches longest balanced parantheses in string.
        """
        if position <= 0:
            return 0
        elif position not in memo:
            value = 0
            if string[position] == ')':
                if string[position-1] == '(':
                    value = 2 + longest_valid_parentheses_recursive(memo, string, position-2)
                else:
                    n_1 = longest_valid_parentheses_recursive(memo, string, position-1)
                    if position-n_1-1 >= 0 and string[position-1-n_1] == '(':
                        value = 2 + n_1 + longest_valid_parentheses_recursive(memo, string, position-n_1-2)
            memo[position] = value
        return memo[position]

    memo, max_value = {}, 0
    for x in range(len(string)-1, 0, -1):
        if x not in memo:
            longest_valid_parentheses_recursive(memo, string, x)
    for index, value in memo.items():
        max_value = max(max_value, value)
    return max_value

def rob(nums: list) -> int:
    """
    Determines the max cost that robber can steal from neighborhood.
    >>> rob([1, 2, 3, 1])
    4
    >>> rob([2, 1, 1, 2])
    4
    >>> rob([3, 1, 1, 1, 5, 1, 6])
    15
    """
    def robbing(memo, nums, pos):
        if pos < 0:
            return 0
        elif pos == 0:
            memo[0] = nums[0]
        elif pos not in memo:
            memo[pos] = max(robbing(memo, nums, pos-1), nums[pos] + robbing(memo, nums, pos-2))
        return memo[pos]
    return robbing({}, nums, len(nums)-1)

def tiling_problem(n: int) -> int:
    """
    Count number of ways to fill 2x1 tiles in a
    2xn grid.
    Fibonacci series
    >>> tiling_problem(4)
    5
    """
    dp = [0, 1]
    for x in range(2, n+2):
        dp.append(dp[-2] + dp[-1])
    return dp[-1]

def coin_change(coins: list[int], amount: int) -> int:
    """
    Determines the minimum amount of coins required to
    add upto the target sum.
    >>> coin_change([1, 2, 5], 11) # 5, 5, 1
    3
    >>> coin_change([186, 419, 83, 408], 6249)
    20
    """
    def coin_change_recursion(memo, w, n):
        if n <= 0:
            return 0 if n == 0 else -1
        elif n in memo:
            return memo[n]
        else:
            min_value = -1
            # From set of coins find the selection that will result 
            # in minimum amount, so that it's easy to refer to 
            # in the future
            for coin in w:
                value1 = coin_change_recursion(memo, w, n - coin)
                if value1 != -1:
                    if min_value == -1:
                        min_value = value1 + 1
                    else:
                        min_value = min(min_value, value1 + 1)
                
            memo[n] = min_value
            return memo[n]
        
    coins.sort(reverse=True) # Not required, but still
    memo = {}
    ans = coin_change_recursion(memo, coins, amount)
    return memo[amount] if amount in memo else 0

def subset_sum_divisibility(array: list, m: int) -> bool:
    """
    Check whether there is a subset with sum divisible by zero.
    >>> subset_sum_divisibility([3, 1, 7, 5], 6)
    True
    >>> subset_sum_divisibility([3, 1, 7, 4], 6)
    True
    >>> subset_sum_divisibility([3, 1, 7], 6)
    False
    >>> subset_sum_divisibility([4, 6, 8, 8, 10, 10, 2, 3, 8, 14, 14, 3], 8)
    True
    """
    def subset_sum_memo(memo: dict, array: list, sum: int, m: int, index: int) -> bool:
        if sum and sum % m == 0:
            return True
        elif index >= len(array):
            return False
        else:
            value1, value2 = False, False
            if sum % m not in memo:
                value1 = subset_sum_memo(memo, array, sum + array[index], m, index + 1)
                value2 = subset_sum_memo(memo, array, sum, m, index + 1)
                if sum:
                    memo[sum % m] = value1 or value2
            if not sum:
                return value1 or value2
            return memo[sum % m]

    if len(array) > m:
        return True
    memo = {}
    return subset_sum_memo(memo, array, 0, m, 0)

if __name__ == '__main__':
    from doctest import testmod
    testmod()
