#include <stdio.h>
#include <immintrin.h>
#include <time.h>

#define uint64_t unsigned long long

#define _MM256_OP_CASTED(INDEX, LOAD_OP, OP, STORE_OP, CASTED) \
    frst = LOAD_OP((CASTED)&alloc[INDEX]); \
    sec = LOAD_OP((CASTED)&alloc[INDEX]); \
    STORE_OP((CASTED)&ans[INDEX], OP(frst, sec));

#define PERFORM_MM256_CASTED(INDEX, LOAD_OP, OP, STORE_OP, SKP, CASTED) \
    _MM256_OP_CASTED(INDEX, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 2, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 3, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 4, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 5, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 6, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 7, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 8, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 9, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 10, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 11, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 12, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 13, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 14, LOAD_OP, OP, STORE_OP, CASTED) \
    _MM256_OP_CASTED(INDEX + SKP * 15, LOAD_OP, OP, STORE_OP, CASTED) \

int main () {
    int size = 1200000000, i;
    int *alloc = (int*)malloc(size*sizeof(int));
    
    for (i = 0; i < size; ++i) alloc[i] = 1;

    clock_t start, end;
    double clock_used = 0;
    start = clock();
    {   
        
        int *ans = (int*)malloc(size*sizeof(int));
        for (i = 0; i < size; ++i) ans[i] = 0;
        __m256i frst, sec;
        for (int i = 0; i < size; i += 128) {
            PERFORM_MM256_CASTED(i, _mm256_loadu_si256, _mm256_add_epi32, _mm256_storeu_si256, 8, __m256i*)
        }
    }
    end = clock();
    clock_used = ((double) (end - start) / CLOCKS_PER_SEC);

    printf("%fs", clock_used);
}
