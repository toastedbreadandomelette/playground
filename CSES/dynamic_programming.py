def dice_combinations(n: int) -> int:
    """
    >>> dice_combinations(50)
    660641036
    >>> dice_combinations(1000)
    937196411
    >>> dice_combinations(654321)
    615247550
    """
    if n <= 6:
        return 1 << (n - 1)
    memo = [0] + [(1 << (n - 1)) for n in range(1, 7)]
    for q in range(7, n + 1):
        memo.append((memo[-1] + memo[-2] + memo[-3] + memo[-4] + memo[-5] + memo[-6]) % 1000000007)
    return memo[-1]

def minimizing_coins(n: int, x: int, c: list) -> int:
    """
    >>> minimizing_coins(3, 11, [1, 5, 7])
    3
    """
    c.sort()
    minimal_coins = [0] + [100000000 for i in range(1, x + 1)]
    for st in c:
        for val in range(st, x + 1):
            minimal_coins[val] = min(
                minimal_coins[val], minimal_coins[val - st] + 1)
    return minimal_coins[x] if minimal_coins[x] <= x else -1

if __name__ == "__main__":
    from doctest import testmod
    testmod()
