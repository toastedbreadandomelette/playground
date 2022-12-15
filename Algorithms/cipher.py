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

def xor_cipher(string: str, key: str) -> str:
    """
    A type of symmetric encryption method (here same key can be used to encrypt and 
    decrpyt the data.)
    >>> print(xor_cipher('Thequickbrownfoxjumpsoverlazydog', '12345678'))
    eZVE@_TSS@\C[PX@[G^DFYA]C^RNLRX_
    >>> print(xor_cipher('eZVE@_TSS@\C[PX@[G^DFYA]C^RNLRX_', '12345678'))
    Thequickbrownfoxjumpsoverlazydog
    """
    encoded_str = ''
    for x in range(0, len(string), len(key)):
        encoded_str += ''.join([chr(ord(string[x + y]) ^ ord(key[y])) for y in range(len(key)) if x + y < len(string)])
    return encoded_str

def caesar_cipher(string: str, shft_by: int) -> str:
    """
    caesar cipher is a substitute cipher, where each character is replaced 
    by the nth next character, cyclic in nature
    >>> caesar_cipher('Hello, world', 13)
    'Uryyb, jbeyq'
    >>> caesar_cipher('Hello, world', 6)
    'Nkrru, cuxrj'
    """
    is_lowercase = lambda a: ord(a) >= 97 and ord(a) <= 97+25
    is_uppercase = lambda a: ord(a) >= 65 and ord(a) <= 65+25
    substitute = lambda a: chr(97 + (ord(a) - 97 + shft_by) % 26) if is_lowercase(a) else (chr(65 + (ord(a) - 65 + shft_by) % 26) if is_uppercase(a) else a)
    return "".join([substitute(char) for char in string])

def vignere_cipher(string: str, keyword: str) -> str:
    """
    A substitution cipher that uses vignere table to substitute
    characters.
    >>> vignere_cipher('attackatdawn', 'LEMONLEMONLE')
    'LXFOPVEFRNHR'
    """
    is_lowercase = lambda a: ord(a) >= 97 and ord(a) <= 97+25
    is_uppercase = lambda a: ord(a) >= 65 and ord(a) <= 65+25
    def substitute(char, key):
        """
        """
        key_case, char_case = 97 if is_lowercase(key) else 65, 97 if is_lowercase(char) else 65
        return chr(key_case + ((ord(char) - char_case) + (ord(key) - key_case)) % 26)
    return "".join([substitute(char, keyword[index % len(keyword)]) for index, char in enumerate(string)])

def rail_fence_decipher(string: str, key: int) -> str:
    """
    Decipher the zig-zag into it's normal form

    >>> rail_fence_decipher('hloel', 2)
    'hello'
    >>> rail_fence_decipher('HWe olordll', 4)
    'Hello World'
    """
    if key == 1:
        return string
    else:
        length = len(string)
        stride = 2*(key - 1)
        out = ['' for x in range(length)]
        index = 0
        for x in range(0, length, stride):
            out[x] = string[index]
            index += 1
        
        for x in range(1, key - 1):
            for y in range(x, length, stride):
                out[y] = string[index]
                index += 1
                
                if y + 2 * (key - 1 - x) < length:
                    out[y + 2 * (key - 1 - x)] = string[index]
                    index += 1
        
        for x in range(key-1, length, stride):
            out[x] = string[index]
            index += 1

        return ''.join(out)

def rail_fence_cipher(string: str, key: int) -> str:
    """
    Encrypt the data in zig-zag fashion

    >>> rail_fence_cipher('hello', 2)
    'hloel'
    >>> rail_fence_cipher('Hello World', 4)
    'HWe olordll'
    """
    if key == 1:
        return string
    elif key == 2:
        return string[::2]+string[1::2]
    else:
        stride = 2*(key - 1)
        out = string[::stride]
        for x in range(1, key - 1):
            for y in range(x, len(string), stride):
                out += string[y]
                if y + 2 * ((key - 1) - x) < len(string):
                    out += string[y + 2 * ((key - 1) - x)]
        return out + string[key-1::stride]

if __name__ == "__main__":
    from doctest import testmod
    testmod()
