```cpp
// CPP
int add(int *ptr, int n) {
    int sum = 0;
    for (int i = 0; i < n; ++i) {
        sum += ptr[i];
    }
    return sum;
}

// assembly
add(int*, int):
    test    esi, esi
    jle     .L7
    lea     eax, [rsi-1]
    cmp     eax, 2
    jbe     .L8
    mov     edx, esi
    mov     rax, rdi
    pxor    xmm0, xmm0
    shr     edx, 2
    sal     rdx, 4
    add     rdx, rdi
.L4:
    movdqu  xmm2, XMMWORD PTR [rax]         // Moves double QUADWORD Unaligned from address stored in rax (SSE - 128-bit by default?)
    add     rax, 16                         // Skips 4 numbers
    paddd   xmm0, xmm2
    cmp     rdx, rax
    jne     .L4                             // Packed SIMD-Loop Jump back to first one. 
    movdqa  xmm1, xmm0
    mov     edx, esi
    psrldq  xmm1, 8
    and     edx, -4
    paddd   xmm0, xmm1
    movdqa  xmm1, xmm0
    psrldq  xmm1, 4
    paddd   xmm0, xmm1
    movd    eax, xmm0
    test    sil, 3
    je      .L11
.L3:
    movsx   rcx, edx
    lea     r8d, [rdx+1]
    add     eax, DWORD PTR [rdi+rcx*4]
    cmp     esi, r8d
    jle     .L1
    add     edx, 2
    add     eax, DWORD PTR [rdi+4+rcx*4]
    cmp     esi, edx
    jle     .L1
    add     eax, DWORD PTR [rdi+8+rcx*4]
    ret
.L7:
    xor     eax, eax
.L1:
    ret
.L11:
    ret
.L8:
    xor     edx, edx
    xor     eax, eax
    jmp     .L3
```