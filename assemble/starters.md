## Basic operations (Add, sub, mul, div)

```cpp
int add(int a, int b) {
    return a + b;
}

int sub(int a, int b) {
    return a - b;
}

int mul(int a, int b) {
    return a * b;
}

int div(int a, int b) {
    return a / b;
}

add(int, int):
    lea     eax, [rdi+rsi]
    ret
sub(int, int):
    mov     eax, edi
    sub     eax, esi
    ret
mul(int, int):
    mov     eax, edi
    imul    eax, esi
    ret
div(int, int):
    mov     eax, edi
    cdq                # convert double to quad.
    idiv    esi
    ret
```

## Basic conditional returning

```cpp
// CPP
int max(int a, int b) {
    return a > b ? a : b;
}

int min(int a, int b) {
    return a > b ? b : a;
}

// Assembly
max(int, int):
    cmp     edi, esi      // Compares first and second args and stores in Flag.
                          // There are Zero Flag, Sign Flag, Carry Flag, and Overflow Flag that would make this comparison.
    mov     eax, esi
    cmovge  eax, edi      // Compare and move if greater than or equal to from edi to eax (refers from sign flag, and moves second arg).
    ret
min(int, int):
    cmp     edi, esi
    mov     eax, esi
    cmovle  eax, edi
    ret
```

## Using above but sending reference.

```cpp
// CPP
int max(int &a, int &b) {
    return a > b ? a : b;
}

// Assembly
max(int&, int&):
    mov     eax, DWORD PTR [rdi]      // RDI now is a memory address, moves value from that address in eax
    mov     edx, DWORD PTR [rsi]
    cmp     eax, edx
    cmovl   eax, edx
    ret
```

## Switch case
```cpp
// CPP
char grade(unsigned marks) {
    switch (marks) {
        case 90 ... 100:
            return 'A';
        case 75 ... 89:
            return 'B';
        case 50 ... 74:
            return 'C';
        case 0 ... 49:
            return 'D';
        default:
            return 'I';
    }
}

// Assembly
grade(unsigned int):
    cmp     edi, 89
    ja      .L6       // Jump if above
    mov     eax, 66   // 'B' ASCII
    cmp     edi, 74
    ja      .L5       // Jump if above
    cmp     edi, 50
    setb    al        // Set AL to 1 if edi was below 50.
    add     eax, 67   // 'C' ASCII (if al before was set to 1, then 'C' + 1 would be 'D')
    ret
.L6:
    sub     edi, 90   // Sub by 90
    cmp     edi, 11   // 
    sbb     eax, eax
    and     eax, -8
    add     eax, 73
.L5:
    ret
```
