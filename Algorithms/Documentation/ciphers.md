# Ciphers

## ROT-13 algorithm

String is replaced with the $13^{\textrm{th}}$ next character in the alphabet series.
e.g.,

```
a b c d e f g h i j k l m n o p q r s t u v w x y z
| | | | | | | | | | | | | | | | | | | | | | | | | |   <= Replaced with
n o p q r s t u v w x y z a b c d e f g h i j k l m
```

```python
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
```