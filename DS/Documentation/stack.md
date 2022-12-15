# Stack (LIFO)
Stack is a type of LIFO (Last In - First Out) Data Structure: meaning the last item to put in it is the one that would be the first to be removed. 

It's a black box, and we only have access to a recently added element.

## Types of stack implementation:
- [[arrays_1d|Dynamic Arrays]]
- [[linked_list|Linked List]]

## Dynamic Arrays
Dynamic arrays can be used to create a stack blueprint. Although any element of array can be accessed, stack should restrict the usage to only last element.

```python
class stack:
	"""
	Stack class
	"""
	def __init__(self):
		self.__stack = [] # make this a private so that no external would have access to all elements

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
		if len(__stack):
			element = self.__stack[-1]
			self.__stack.pop()
			return element
		raise Exception('Peeking an empty stack')

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
```

## [[linked_list|Linked List]]
Linked list can also act as a stack, where head will be a top of stack to push, pop and value peek.
