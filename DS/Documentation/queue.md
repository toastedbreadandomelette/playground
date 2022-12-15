# Queues
#Queues are an example of #FIFO (First In-First Out) Data Structure: It's restricted to remove element that is inserted first into.

# Types of Queue Implementation:
## [[arrays_1d|Arrays]]
- Arrays can be used by keeping track of two pointers: start $s$ and end $e$, in a circular way.
 $$
\begin{array}{ c l }
Q&=&
[\ &&&2&4&1&7&8&9\ ]\\
&&&&&\uparrow&&&&&\uparrow\\
&&&&&\text{s}&&&&&\text{e}
\end{array}
$$
- While adding, the last pointer $e$ will add and increment pointer (will return to $0$ if it exceeds the size of array allocation)
$$
\begin{array}{ c l }
Q&=&
[\ 1&&&2&4&1&7&8&9\ ]\\
&&\ \uparrow&&&\uparrow\\
&&\ \text{ e}&&&\text{s}
\end{array}
$$
- The first pointer $s$ handle popping off value and incrementing. (will return to $0$ if it goes out of array allocation).
$$
\begin{array}{ c l }
Q&=&
[\ 1&&&&4&1&7&8&9\ ]\\
&&\ \uparrow&&&&\uparrow\\
&&\ \text{ e}&&&&\text{s}
\end{array}
$$


## [[linked_list|Linked List]]
- Linked list are more reliable way to allocate and deallocate as needed.
- A modified linked list involving tail pointer will handle newly inserted nodes, and head pointer will handle removal of the node.
- Unlike arrays, the cap for queue size is the available memory size.