class stack:
    """
    Stack class
    """
    def __init__(self):
        # make this a private so that no external would have access to all elements
        self.__stack = []

    def __iadd__(self, value):
        """
        equivalent of stack.push(value)
        instead we do stack += value
        """
        self.__stack.append(value)
        return self

    def peek(self):
        """
        return the last inserted element
        """
        if len(self.__stack):
            return self.__stack[-1]
        raise Exception('Peeking an empty stack')

    def pop(self):
        """
        return the last inserted element
        """
        if len(self.__stack):
            element = self.__stack[-1]
            self.__stack.pop()
            return element
        raise Exception('Poping from an empty stack')

    def empty(self):
        """
        returns true if stack is empty
        """
        return len(self) == 0

    def __len__(self):
        """
        return the size of the stack
        """
        return len(self.__stack)

if __name__ == '__main__':
    from doctest import testmod
    testmod()
