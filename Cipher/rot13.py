def rot13(string: str) -> str:
    """
    Rotate the string by 13
    >>> rot13('abcdef')
    'nopqrs'
    >>> rot13('Hello, world')
    'Uryyb, jbeyq'
    """
    is_lowercase = lambda a: ord(a) >= 97 and ord(a) <= 97+25
    is_uppercase = lambda a: ord(a) >= 65 and ord(a) <= 65+25
    substitute = lambda a: chr(97 + (ord(a) - 97 + 13) % 26) if is_lowercase(a) else (chr(65 + (ord(a) - 65 + 13) % 26) if is_uppercase(a) else a)
    return "".join([substitute(char) for char in string])

if __name__ == "__main__":
    from doctest import testmod
    testmod()
