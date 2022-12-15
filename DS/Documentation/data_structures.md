# Table of complexities
## [[arrays_1d#Playground for Array note observation|Array]]
|Operations|Best-case Complexity|Worst-case Complexity|
|-|-|-|
|Insert/Delete at position $i$|$O(1)$, if $i=n$ |$O(n)$, if $i=0$|
|Access/Modify $n^{th}$ element|$O(1)$|$O(1)$|
|Append|$O(1)$ (amortised)|$O(1)$|
|Search element|$O(1)$|$O(n)$ (in unsorted array), $O(\log_2{n})$ for sorted arrays (Interpolation search might take $O(\log{\log{n}})$ time)|
|Reverse/shift/Rotate element|$O(n)$|$O(n)$|

## [[linked_list|Linked List]]

|Operations|Best-case Complexity|Worst-case Complexity|
|-|-|-|
|Insert/Delete $n^{th}$ element|$O(1)$, if $i=0$|$O(n)$, if $i=n$|
|Access/Modify $n^{th}$ element|$O(1) \iff i=0$|$O(n)$$\iff i=n$|
|Append|$O(n)$ (if there is no tail pointer, else $O(1)$)|$O(n)$ (if there is no tail pointer, else $O(1)$)|
|Search element|$O(1)$|$O(n)$|
|Reverse/shift/Rotate element|$O(n)$|$O(n)$|


## [[graphs_1|Graphs]]

## [[trees|Trees]]

## [[queue]]

## [[stack]]

## [[heaps]]

