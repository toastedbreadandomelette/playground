# Multi-dimensional array.
In practice, keeping a $n$-dimensional array can be infeasible:

An array having an arbitrary $n$ dimension size with arbitrary shape, say tuple $S = (s_1, s_2,\dots,s_n), \forall\ i, s_i\in \textrm{N}$, needs to have n nested pointers.

```cpp
int ********arr; // Hell.
```

The code cannot be written that feasibly since we need that information dynamically whether an inner data contains either a list of pointer or the data that you need.

```cpp
enum Kind {
    Data,
    Nested,
}
union MaybeArray<T> {
    Kind kind;
    void **pointers; // For nested
    T *arr; // for actual content
};
```

There are problems with the approach.
- It is tough to manage memory (e.g., deleting memory).
- Although there is some flexibility of extending arrays individually, There are multiple jumps to perform before you even get to the actual memory location and read the data.
- Creating or copying an array with enormous hierarchy is expensive since `malloc` or `new` is expensive.

In most cases like fast tensor processing or scientific calculation, we do not use such method. Instead we have a single pointer and a dimension tuple.

```cpp
struct MultidimensionalArray<T> {
    T *array;
    std::vector<size_t> shape;  // The $S$ described above.
};
```

We usually deal with more static shaped $n$-dimensional array than extensible dynamic multi-dimensional array. This creates a more cache-friendly access to each elements.
